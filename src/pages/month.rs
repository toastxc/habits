use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaMinimize, FaPenToSquare, FaPlus},
    Icon,
};

use crate::{
    backend::{
        middle::{date_get, entries_get, habits_get},
        Habit,
    },
    components::DaysOfWeek,
    Route,
};

#[component]
pub fn Month(id: u32) -> Element {
    let Some((date_info, habits)) =
        use_resource(
            move || async move { (date_get().await.unwrap(), habits_get(id).await.unwrap()) },
        )
        .read_unchecked()
        .clone()
    else {
        return rsx! { "loading..." };
    };

    let month_start = date_info.start_of_this_week - 21;

    let habits = use_signal(|| habits);

    let habits_rendered = habits.read().clone().into_iter().map(|(habit_id, habit)| {
        //    date_info.
        let month = rsx! {

            Week { date: month_start, habit: habit.clone(), habit_id }
        };

        rsx! {

            div { class: "boxes",
                p { class: "box", {habit.name.clone()} }
            }

            DaysOfWeek { offset: false }
            {month}
        }
    });

    rsx! {
        div { class: "scroller", id: "aa", {habits_rendered} }
        br {}
    }
}

#[component]
fn Week(date: u32, habit: Habit, habit_id: u32) -> Element {
    let Some(mut entries) = use_resource(move || async move {
        let days = (0..28).map(|day| day + date).collect();

        entries_get(habit_id, days)
            .await
            .unwrap()
            .into_iter()
            .map(|value| {
                rsx! {
                    BoxerNon { value }
                }
            })
            .collect::<Vec<_>>()
    })
    .read_unchecked()
    .clone() else {
        return rsx! {};
    };

    rsx! {

        div { class: "boxes", {(0..7).map(|_| entries.remove(0))} }
        div { class: "boxes", {(0..7).map(|_| entries.remove(0))} }
        div { class: "boxes", {(0..7).map(|_| entries.remove(0))} }
        div { class: "boxes", {(0..7).map(|_| entries.remove(0))} }
    }
}

#[component]
fn BoxerNon(value: bool) -> Element {
    let class = use_signal(|| if value { "box  bx-fill" } else { "box bx-line" });
    rsx! {
        a { class, style: "width: 12.88%;" }
    }
}
