use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaMinimize, FaPenToSquare, FaPlus},
    Icon,
};

use crate::{
    backend::{user::user_get, Habit},
    components::DaysOfWeek,
    Route,
};

#[component]
pub fn Month(id: u32) -> Element {
    let Some(Some(user)) = use_resource(move || async move { user_get(id).await.unwrap() })
        .read_unchecked()
        .clone()
    else {
        return rsx! {
            h1 { "404" }
        };
    };
    let user = use_signal(|| user);

    // let mut user = user.clone();

    let habits_rendered = user.read().clone().habits.into_iter().map(|(name, habit)| {
        let month = (0..4).map(|day| {
            // let habits_rendered = Date::Cu .map(|(name, habit)| {
            rsx! {

                Week { date: day, habit: habit.clone() }
            }
        });

        rsx! {



            div { class: "boxes",
                p { class: "box", {name} }
            }

            DaysOfWeek { offset: false }
            {month}
        }
    });

    rsx! {

        {habits_rendered}



        div { class: "boxes",
            div { class: "box bx-line is-disabled",
                Icon { icon: FaPlus }
            }


            div { class: "box bx-line is-disabled",
                Icon { icon: FaPenToSquare }
            }


            Link { to: Route::U { id }, class: "box bx-line ",
                Icon { icon: FaMinimize }
            }
        }
    }
}

#[component]
fn Week(date: u32, habit: Habit) -> Element {
    rsx! {


        div { class: "boxes",

            div {
                // p { class: "box", style: "width:70px;", {habit.name} }
            }

            BoxerNon { value: false }
            BoxerNon { value: false }
            BoxerNon { value: false }
            BoxerNon { value: false }
            BoxerNon { value: false }
            BoxerNon { value: false }
            BoxerNon { value: false }
        }
    }
}

#[component]
fn BoxerNon(value: bool) -> Element {
    let class = use_signal(|| if value { "box  bx-fill" } else { "box bx-line" });
    rsx! {
        a { class }
    }
}
