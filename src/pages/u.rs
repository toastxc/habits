use crate::backend::db_save;
use crate::backend::entry::entry_update;
use crate::backend::habit::habit_make;
use crate::backend::time::date_get;
use crate::backend::time::DateInfo;
// use crate::backend::user;
use crate::backend::user::user_get;
use crate::backend::Habit;
use crate::backend::User;
use crate::components::field::TextInput;
use crate::components::DaysOfWeek;
use crate::Route;
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaExpand;
use dioxus_free_icons::icons::fa_solid_icons::FaPenToSquare;
use dioxus_free_icons::icons::fa_solid_icons::FaPlus;
use dioxus_free_icons::Icon;

#[component]
pub fn U(id: u32) -> Element {
    let Some((Some(user), date_info)) =
        use_resource(
            move || async move { (user_get(id).await.unwrap(), date_get().await.unwrap()) },
        )
        .read_unchecked()
        .clone()
    else {
        return rsx! {
            h1 { "404" }
        };
    };
    let user = use_signal(|| user);

    let habits_rendered = user.read().clone().habits.into_values().map(|habit| {
        rsx! {

            Week {
                habit: habit.clone(),
                date_info: date_info.clone(),
                user_id: id,
            }
        }
    });

    warn!("{:?}", user);

    let mut dialog_open = use_signal(|| false);
    rsx! {
        if dialog_open() {
            Dialog { open: dialog_open, user }
        }


        br {}
        DaysOfWeek { offset: true }
        {habits_rendered}

        div { class: "boxes",
            a {
                class: "box bx-line",
                onclick: move |_| { dialog_open.set(true) },
                Icon { icon: FaPlus }
            }


            Link { to: Route::Edit { id }, class: "box bx-line",
                Icon { icon: FaPenToSquare }
            }




            Link { to: Route::Month { id }, class: "box bx-line",
                Icon { icon: FaExpand }
            }
        }
    }
}

#[component]
fn Week(habit: Habit, date_info: DateInfo, user_id: u32) -> Element {
    let (m, t, w, th, f, sa, su) = (
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
    );

    rsx! {
        div { class: "boxes" }

        div { class: "boxes",

            div {
                p { class: "box", style: "width:70px;", {habit.name.clone()} }
            }



            Boxer {
                value: m,
                date: date_info.start_of_this_week,
                habit_name: habit.name,
                user_id,
            }
                // Boxer { value: t, date_info }
        // Boxer { value: w, date_info }
        // Boxer { value: th, date_info }
        // Boxer { value: f, date_info }
        // Boxer { value: sa, date_info }
        // Boxer { value: su, date_info }
        }
    }
}

#[component]
fn Boxer(value: Signal<bool>, date: u32, habit_name: String, user_id: u32) -> Element {
    let bool_value = *value.read();
    let mut class = use_signal(|| {
        if *value.read() {
            "box  bx-fill"
        } else {
            "box bx-line"
        }
    });
    rsx! {
        a {
            class,
            onclick: move |_| {
                let habit_name = habit_name.clone();
                async move {
                    value.set(!bool_value);
                    class.set(if *value.read() { "box  bx-fill" } else { "box bx-line" });
                    entry_update(user_id, habit_name, date).await.unwrap();
                    db_save().await.unwrap();
                }
            },
        }
    }
}
#[component]
fn Dialog(open: Signal<bool>, user: Signal<User>) -> Element {
    let text = use_signal(String::new);

    let button = rsx! {
        a {
            class: "is-bordered",
            onclick: move |_| async move {
                let id = user.read().id;
                let habit = habit_make(id, text.to_string()).await.unwrap();
                user.write().habits.insert(text.to_string(), habit);
                habit_make(id, text.to_string()).await.unwrap();
                db_save().await.unwrap();
                open.set(false);
            },
            "Create"
        }
    };
    rsx! {

        div { class: "dialog",

            div { class: "dialog-content",

                a {
                    class: "topright is-bordered",
                    onclick: move |_| { open.set(false) },

                    "X"
                }
                h3 { "Habit name: " }
                TextInput { text, button }
            }
        }
    }
}
