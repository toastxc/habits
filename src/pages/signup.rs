use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaChevronRight, Icon};
use std::ops::Deref;

use crate::backend::middle::db_save;
use crate::backend::middle::user_make;
use crate::disable;

use crate::Route;
#[component]
pub fn Signup() -> Element {
    let mut id = use_signal(|| "".to_string());

    rsx! {
        h2 { "This is your code:" }
        div { style: "display: flex;
    flex-wrap: nowrap; space-evenly",


            Icon { style: "padding: 5px;", icon: FaChevronRight }
            input {
                class: "input is-underlined {disable(false)}",
                r#type: "text",
                value: id,
            }
            // h4 {

            if id.read().is_empty() {

                a {
                    class: "is-bordered",
                    onclick: move |_| async move {
                        let user_id = user_make().await.unwrap();
                        db_save().await.unwrap();
                        id.set(user_id.to_string());
                    },
                    "Generate"
                }
            } else {
                a { class: "is-bordered", "Copy" }
                Link {
                    to: Route::U {
                        id: id.read().deref().parse::<u32>().unwrap_or_default(),
                    },
                    class: "is-bordered",
                    "Go"
                }
            }
        }

        p { class: "is-leftlined",
            "Psst, make sure to write this code down! or bookmark: https://habbits.toastxc.xyz/u/{id.read().deref()}"
        }
    }
}
