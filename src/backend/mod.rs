pub mod entry;
pub mod habit;
pub mod time;
pub mod user;
use std::collections::{HashMap, HashSet};
#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use std::sync::RwLock;

use dioxus::prelude::*;
#[server]
pub async fn db_save() -> Result<(), ServerFnError> {
    let db = DB.read().unwrap().clone();
    std::fs::write("db.json", serde_json::to_string(&db).unwrap());
    Ok(())
}

#[cfg(feature = "server")]
use lazy_static::lazy_static;
#[cfg(feature = "server")]
lazy_static! {
    static ref EXAMPLE: u8 = 42;
    static ref DB: Arc<RwLock<Data>> = Arc::new(RwLock::new(
        match std::fs::read("db.json").map(|op| serde_json::from_slice(&op)) {
            Ok(Ok(db)) => db,
            _ => {
                std::fs::write("db.json", serde_json::to_string(&Data::default()).unwrap());
                Data::default()
            }
        }
    ));
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Data {
    pub users: HashMap<u32, User>,
}
#[derive(Clone, Default, Deserialize, Serialize, Debug)]

pub struct User {
    pub id: u32,
    pub habits: HashMap<String, Habit>,
}
#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Debug)]
pub struct Habit {
    pub name: String,
    pub entries: HashSet<u32>,
}

use serde::{Deserialize, Serialize};
