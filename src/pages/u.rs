use crate::backend::middle::date_get;
use crate::backend::middle::db_save;
use crate::backend::middle::entries_get;
use crate::backend::middle::entry_toggle;
use crate::backend::middle::habit_make;
use crate::backend::middle::habits_get;
use crate::backend::middle::user_get;
use crate::backend::DateInfo;
// use crate::backend::user;
use crate::backend::Habit;
use crate::backend::User;
use crate::components::field::TextInput;
use crate::components::DaysOfWeek;
use crate::Route;
use dioxus::html::b;
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaExpand;
use dioxus_free_icons::icons::fa_solid_icons::FaPenToSquare;
use dioxus_free_icons::icons::fa_solid_icons::FaPlus;
use dioxus_free_icons::Icon;

#[component]
pub fn U(id: u32) -> Element {
    let Some((Some(user), date_info, habits)) = use_resource(move || async move {
        (
            user_get(id).await.unwrap(),
            date_get().await.unwrap(),
            habits_get(id).await.unwrap(),
        )
    })
    .read_unchecked()
    .clone() else {
        return rsx! {
            h1 { "404" }
        };
    };
    let user = use_signal(|| user);
    let habits = use_signal(|| habits);

    let habits_rendered = habits.read().clone().into_iter().map(|(habit_id, habit)| {
        rsx! {

            Week {
                habit: habit.clone(),
                date_info: date_info.clone(),
                user_id: id,
                habit_id,
            }
        }
    });

    warn!("{:?}", user);

    let mut dialog_open = use_signal(|| false);
    rsx! {
        if dialog_open() {
            Dialog { open: dialog_open, user_id: id, habits }
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




        // Link { to: Route::Month { id }, class: "box bx-line",
        //     Icon { icon: FaExpand }
        // }
        }
    }
}

#[component]
fn Week(habit: Habit, date_info: DateInfo, user_id: u32, habit_id: u32) -> Element {
    // let e = (0..7)
    //     .map(|day| day + date_info.start_of_this_week)
    //     .collect();

    // let entries: Vec<_> = entries_get(habit_id, e)
    //     .await
    //     .unwrap()
    //     .map(|a| use_signal(|| a))
    //     .collect();

    let days: Vec<_> = (0..7)
        .map(|day| day + date_info.start_of_this_week)
        .collect();

    let Some(entries) = use_resource(move || async move {
        let days: Vec<_> = (0..7)
            .map(|day| day + date_info.start_of_this_week)
            .collect();

        entries_get(habit_id, days)
            .await
            .unwrap()
            .into_iter()
            .map(|a| use_signal(|| a))
            .collect::<Vec<_>>()
    })
    .read_unchecked()
    .clone() else {
        return rsx! {
            h1 { "404" }
        };
    };

    // let vec = entries_get(habit_id, );

    let boxes_rendered = (0..7).map(|x| {
        rsx! {

            Boxer {
                value: entries[x],
                date: days[x],
                habit_id,
                user_id,
            }
        }
    });

    rsx! {
        div { class: "boxes" }

        div { class: "boxes",

            div {
                p { class: "box", style: "width:70px;", {habit.name.clone()} }
            }


            {boxes_rendered}

        }
    }
}

#[component]
fn Boxer(value: Signal<bool>, date: u32, habit_id: u32, user_id: u32) -> Element {
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
                async move {
                    value.set(!bool_value);
                    class.set(if *value.read() { "box  bx-fill" } else { "box bx-line" });
                    entry_toggle(habit_id, date).await.unwrap();
                    db_save().await.unwrap();
                }
            },
        }
    }
}
#[component]
fn Dialog(open: Signal<bool>, user_id: u32, habits: Signal<Vec<(u32, Habit)>>) -> Element {
    let text = use_signal(String::new);

    let button = rsx! {
        a {
            class: "is-bordered",
            onclick: move |_| async move {
                let habit_id = habit_make(user_id, text.to_string()).await.unwrap();
                db_save().await.unwrap();
                habits.write().push((habit_id, Habit { name: text.to_string() }));
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
