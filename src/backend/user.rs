use crate::backend::User;
#[cfg(feature = "server")]
use crate::backend::DB;
#[cfg(feature = "server")]
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
#[server]
pub async fn users_get() -> Result<Vec<u32>, ServerFnError> {
    Ok(DB
        .read()
        .unwrap()
        .users
        .keys()
        .cloned()
        .collect::<Vec<u32>>())
}

#[server]
pub async fn user_get(user_id: u32) -> Result<Option<User>, ServerFnError> {
    let a = DB.read().unwrap().users.get(&user_id).cloned();
    if a.is_none() {
        warn!("not found 45654")
    } else {
        warn!("OK ")
    }

    Ok(DB.read().unwrap().users.get(&user_id).cloned())
}

#[server]
pub async fn user_make() -> Result<User, ServerFnError> {
    let mut rng = rand::rng();
    use rand::prelude::*;
    let user = User {
        id: rng.random(),
        ..Default::default()
    };

    DB.write().unwrap().users.insert(user.id, user.clone());
    Ok(user)
}
