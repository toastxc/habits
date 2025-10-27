use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaExpand, FaHouse, FaPlus, FaX},
    Icon,
};

use crate::{
    backend::middle::{db_save, habit_delete, habits_get, user_get},
    Route,
};

#[component]
pub fn Edit(id: u32) -> Element {
    let Some((Some(user), habits)) = use_resource(move || async move {
        (user_get(id).await.unwrap(), habits_get(id).await.unwrap())
    })
    .read_unchecked()
    .clone() else {
        return rsx! {
            h1 { "404" }
        };
    };
    let user = use_signal(|| user);

    let rendered = habits.into_iter().map(|(habit_id, habit)| {
        let name = habit.name;

        rsx! {

            div { class: "boxes",


                div { class: "box bx-line",

                    p { class: "box", style: "justify-content: left;", "{name}" }

                    a {
                        class: "box ",
                        style: "width: 10%; margin-right:-5px;",
                        onclick: move |_| {
                            async move {
                                habit_delete(habit_id, id).await.unwrap();
                                user.clone().write().habits.remove(&habit_id);
                                db_save().await.unwrap();
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

        div { class: "boxes foot",
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
