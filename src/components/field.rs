use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaChevronRight, Icon};

#[component]
pub fn TextInput(text: Signal<String>, button: Element) -> Element {
    rsx! {


        div { style: "display: flex;
    flex-wrap: nowrap; space-evenly",


            Icon { style: "padding: 5px;", icon: FaChevronRight }
            input {
                class: "input is-underlined",
                value: "{text}",
                oninput: move |event| text.set(event.value()),
            }
            {button}
        }
    }
}
