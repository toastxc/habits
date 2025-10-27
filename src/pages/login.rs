use std::ops::Deref;

use dioxus::prelude::*;
// use dioxus_free_icons::{};

use crate::disable;
use crate::{components::field::TextInput, Route};
#[component]
pub fn Login() -> Element {
    let id = use_signal(|| "".to_string());

    let button = rsx! {

        Link {
            to: Route::U {
                id: id.read().deref().parse::<u32>().unwrap_or_default(),
            },
            class: "is-bordered {disable(id.read().deref().parse::<u32>().is_ok())}",
            "Go"
        }
    };

    rsx! {

        h2 { "Enter your code:" }

        TextInput { text: id, button }



        p { hidden: true, class: "is-leftlined",
            "Psst, you can just visit https://habbits.toastxc.xyz/u/{id.read().deref()}"
        }


        br {}
    }
}
