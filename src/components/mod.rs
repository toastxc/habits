pub mod field;

use dioxus::prelude::*;

#[component]
pub fn DaysOfWeek(offset: bool) -> Element {
    rsx! {
        div { class: "boxes",
            if offset {
                div {
                    p { class: "box", style: "width:70px;" }
                }
            }
            div { class: "box", "M" }
            div { class: "box", "T" }
            div { class: "box", "W" }
            div { class: "box", "T" }
            div { class: "box", "F" }
            div { class: "box", "S" }
            div { class: "box", "S" }
        }
    }
}
