#[cfg(feature = "server")]
use crate::backend::db::DB;
use crate::backend::{DateInfo, Habit, User};
use dioxus::prelude::*;

#[server(endpoint = "habits_get")]
pub async fn habits_get(user_id: u32) -> Result<Vec<(u32, Habit)>, ServerFnError> {
    Ok(DB.habits_get_again(user_id))
}

#[server(endpoint = "entry_toggle")]
pub async fn entry_toggle(habit_id: u32, day: u32) -> Result<(), ServerFnError> {
    DB.entry_toggle(habit_id, day);
    Ok(())
}

#[server(endpoint = "user_make")]
pub async fn user_make() -> Result<u32, ServerFnError> {
    Ok(DB.user_make())
}

#[server(endpoint = "user_get")]
pub async fn user_get(id: u32) -> Result<Option<User>, ServerFnError> {
    Ok(DB.user_get(&id))
}

#[server(endpoint = "habit_make")]
pub async fn habit_make(user_id: u32, habit_name: String) -> Result<u32, ServerFnError> {
    Ok(DB.habit_make(&user_id, habit_name))
}

#[server(endpoint = "habit_delete")]
pub async fn habit_delete(habit_id: u32, user_id: u32) -> Result<(), ServerFnError> {
    Ok(DB.habit_delete(&habit_id, &user_id))
}

#[server(endpoint = "date_get")]
pub async fn date_get() -> Result<DateInfo, ServerFnError> {
    Ok(DB.date_get())
}

#[server(endpoint = "db_save")]
pub async fn db_save() -> Result<(), ServerFnError> {
    Ok(DB.save())
}

#[server(endpoint = "entries_get")]
pub async fn entries_get(habit_id: u32, entries: Vec<u32>) -> Result<Vec<bool>, ServerFnError> {
    Ok(DB.entries_get_finite(&habit_id, entries))
}
