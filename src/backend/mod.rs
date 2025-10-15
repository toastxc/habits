#[cfg(feature = "server")]
pub mod db;
pub mod middle;

use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct DataSerde {
    // user id | user data
    pub users: HashMap<u32, User>,
    // user id habit id | habit data
    pub habits: HashMap<u32, Habit>,
    // habit id | entry id timestamp
    pub entries: HashSet<(u32, u32)>,
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]

pub struct User {
    pub habits: HashSet<u32>,
}
#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Debug)]
pub struct Habit {
    pub name: String,
}

#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Debug)]
pub struct DateInfo {
    pub days_since_epoch: u32,
    pub days_since_monday: u32,
    pub start_of_this_week: u32,
}
