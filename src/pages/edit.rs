use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaExpand, FaHouse, FaPlus, FaX},
    Icon,
};

use crate::{
    backend::{habit::habit_delete, user::user_get},
    Route,
};

#[component]
pub fn Edit(id: u32) -> Element {
    let Some(Some(user)) = use_resource(move || async move { user_get(id).await.unwrap() })
        .read_unchecked()
        .clone()
    else {
        return rsx! {
            h1 { "404" }
        };
    };
    let user = use_signal(|| user);

    let rendered = user.read().clone().habits.into_keys().map(|name| {
        rsx! {

            div { class: "boxes",


                div { class: "box bx-line",

                    p { class: "box", style: "justify-content: left;", "{name}" }


                    //       Link { to: Route::U { id }, class: "box ", style:"width: 10%; margin-right:-5px;",
                    //     Icon { icon: FaX }
                    // }

                    a {
                        class: "box ",
                        style: "width: 10%; margin-right:-5px;",
                        onclick: move |_| {
                            let name = name.clone();
                            async move {
                                habit_delete(id, name.clone()).await.unwrap();
                                user.clone().write().habits.remove(&name);
                            }
                        },
                        Icon { icon: FaX }
                    }
                }
            }
        }
    });

    rsx! {


        {rendered}

        div { class: "boxes",
            div { class: "box bx-line is-disabled",
                Icon { icon: FaPlus }
            }


            Link { to: Route::U { id }, class: "box bx-line",
                Icon { icon: FaHouse }
            }


            div { class: "box bx-line is-disabled",
                Icon { icon: FaExpand }
            }
        }
    }
}
