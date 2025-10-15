use std::ops::Deref;

use dioxus::prelude::*;
// use dioxus_free_icons::{};

use crate::{backend::middle::user_get, components::field::TextInput, Route};

#[component]
pub fn Login() -> Element {
    let id = use_signal(|| "".to_string());

    let button = rsx! {

        Link {
            to: Route::U {
                id: id.read().deref().parse::<u32>().unwrap_or_default(),
            },
            class: "is-bordered",
            "Go"
        }
    };

    rsx! {


        TextInput { text: id, button }



        p { hidden: true, class: "is-leftlined",
            "Psst, you can just visit https://habbits.toastxc.xyz/u/{id.read().deref()}"
        }


        br {}



        a {
            class: "is-underlined",
            onclick: move |_| async move {
                let _ = user_get(0).await;
            },
            "aaa"
        }
    }
}
