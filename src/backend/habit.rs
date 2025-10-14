use crate::backend::Habit;
#[cfg(feature = "server")]
use crate::backend::DB;
use dioxus::prelude::*;
#[server]
pub async fn habit_make(user_id: u32, habit_name: String) -> Result<Habit, ServerFnError> {
    if DB.read().unwrap().users.get(&user_id).is_none() {
        return Err(ServerFnError::new(format!("user {} not found!", user_id)));
    }
    let habit = Habit {
        name: habit_name.clone(),
        ..Default::default()
    };

    DB.write()
        .unwrap()
        .users
        .get_mut(&user_id)
        .unwrap()
        .habits
        .insert(habit_name, habit.clone());

    // test
    // warn!("GETTING: {:?}", DB.read().unwrap().users[&user_id].habits.clone());

    Ok(habit)
}

#[server]
pub async fn habit_delete(user_id: u32, habit_name: String) -> Result<(), ServerFnError> {
    if DB.read().unwrap().users.get(&user_id).is_none() {
        return Err(ServerFnError::new(format!("user {} not found!", user_id)));
    }

    DB.write()
        .unwrap()
        .users
        .get_mut(&user_id)
        .unwrap()
        .habits
        .remove(&habit_name);

    Ok(())
}
