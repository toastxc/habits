use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaHouseChimney, FaMaximize, FaPenToSquare, FaPlus},
    Icon,
};

use crate::{
    backend::middle::{db_save, habit_make},
    components::field::TextInput,
    Route,
};

#[component]
pub fn Navbar(id: u32) -> Element {
    let width = 30;
    let height = 30;

    let mut dialog_open = use_signal(|| false);

    let class = "box bx-line bx-black";
    rsx! {

        if dialog_open() {
            Dialog { open: dialog_open, user_id: id }
        }

        Outlet::<Route> {}


        div { class: "boxes foot",
            a { class, onclick: move |_| { dialog_open.set(true) },
                Icon { width, height, icon: FaPlus }
            }


            Link { to: Route::U { id }, class,
                Icon { icon: FaHouseChimney, width, height }
            }

            Link { to: Route::Edit { id }, class,
                Icon { width, height, icon: FaPenToSquare }
            }

            Link { to: Route::Month { id }, class,
                Icon { width, height, icon: FaMaximize }
            }
        }
    }
}

#[component]
fn Dialog(open: Signal<bool>, user_id: u32) -> Element {
    let text = use_signal(String::new);

    let button = rsx! {
        a {
            class: "is-bordered",
            onclick: move |_| async move {
                let _ = habit_make(user_id, text.to_string()).await.unwrap();
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
