#[cfg(feature = "server")]
use crate::backend::DB;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::logger::tracing::warn;

#[server]
pub async fn entry_update(
    user_id: u32,
    habit_id: String,
    entry_id: u32,
) -> Result<(), ServerFnError> {
    let db = DB.read().unwrap();

    // check if container exists
    let Some(user) = db.users.get(&user_id) else {
        return Err(ServerFnError::new("User not found!"));
    };
    let Some(habit) = user.habits.get(&habit_id) else {
        return Err(ServerFnError::new(format!(
            "habit for user {} not found!",
            user_id
        )));
    };

    warn!("passed checklist");
    // update value

    if DB.read().unwrap().users[&user_id].habits[&habit_id]
        .entries
        .get(&entry_id)
        .is_some()
    {
        warn!("1");
        let mut db = DB.write().unwrap().users[&user_id].habits[&habit_id]
            .entries
            .clone();
        db.remove(&entry_id);
    } else {
        warn!("2");

        let mut habit = DB.read().unwrap().users[&user_id].habits[&habit_id].clone();
        warn!("3");
        habit.entries.insert(entry_id);
        warn!("4");


        // warn!("locked: {}", DB.locked());

        DB.write()
            .unwrap()
            .users
            .get_mut(&user_id)
            .unwrap()
            .habits
            .insert(habit.name.clone(), habit);


        // .entries
        // .insert(entry_id);
        warn!("5");
    }

    Ok(())
}

#[server]
pub async fn entry_get(user_id: u32, habit_id: String, date: u32) -> Result<bool, ServerFnError> {
    let db = DB.read().unwrap();
    // check if container exists
    let Some(user) = db.users.get(&user_id) else {
        return Err(ServerFnError::new("User not found!"));
    };
    let Some(habit) = user.habits.get(&habit_id) else {
        return Err(ServerFnError::new(format!(
            "habit for user {} not found!",
            user_id
        )));
    };

    Ok(DB.read().unwrap().users[&user_id].habits[&habit_id]
        .entries
        .get(&date)
        .is_some())
}
