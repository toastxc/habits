use dioxus::prelude::*;

use crate::Route;
#[component]
pub fn NotFound(thing: Option<String>, id: Option<String>) -> Element {
    match (thing, id) {
        (Some(thing), None) => rsx!(Generic {
            e_code: "Not Found",
            e_message: "Couldn't find resource \"{thing}\"",
        }),
        (Some(thing), Some(id)) => rsx!(Generic {
            e_code: "Not Found",
            e_message: "Couldn't find {thing} \"{id}\"",
        }),
        _ => rsx!(Generic {
            e_code: "Not Found",
            e_message: ""
        }),
    }
}

#[component]
pub fn InternalServerError(error: ServerFnError) -> Element {
    rsx!(Generic {
        e_code: "Internal Server Error",
        e_message: error.to_string()
    })
}

#[component]
fn Generic(e_code: String, e_message: String) -> Element {
    rsx! {
        h1 { "{e_code} :/" }
        h3 { "{e_message}" }
        Link { to: Route::Home {}, class: "",
            h3 { class: "is-bordered", "Home" }
        }
    }
}

#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    rsx! {
        NotFound { thing: segments.first().cloned() }
    }
}
