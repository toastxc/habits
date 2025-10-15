use crate::backend::DataSerde;
use crate::backend::DateInfo;
use crate::backend::Habit;
use crate::backend::User;
use lazy_static::lazy_static;
use rand::prelude::*;
use rand::random;
use std::fs;
use std::sync::Arc;
use std::sync::RwLock;
lazy_static! {
    pub static ref DB: Data =
        match std::fs::read("db.json").map(|op| serde_json::from_slice::<DataSerde>(&op)) {
            Ok(Ok(db)) => db.into(),
            _ => {
                std::fs::write(
                    "db.json",
                    serde_json::to_string(&DataSerde::default()).unwrap(),
                )
                .unwrap();
                Data::default()
            }
        };
}

use hashbrown::HashMap;
use hashbrown::HashSet;

#[derive(Clone, Default, Debug)]
pub struct Data {
    users: Arc<RwLock<HashMap<u32, User>>>,
    habits: Arc<RwLock<HashMap<u32, Habit>>>,
    entries: Arc<RwLock<HashSet<(u32, u32)>>>,
}

impl From<Data> for DataSerde {
    fn from(value: Data) -> Self {
        Self {
            users: value.users.read().unwrap().clone(),
            habits: value.habits.read().unwrap().clone(),
            entries: value.entries.read().unwrap().clone(),
        }
    }
}

impl From<DataSerde> for Data {
    fn from(value: DataSerde) -> Self {
        Self {
            users: Arc::new(RwLock::new(value.users)),
            habits: Arc::new(RwLock::new(value.habits)),
            entries: Arc::new(RwLock::new(value.entries)),
        }
    }
}

impl Data {
    pub fn json(self) -> String {
        let db: DataSerde = self.into();

        serde_json::to_string_pretty(&db).unwrap()
    }

    pub fn save(&self) {
        fs::write("db.json", self.clone().json()).unwrap();
    }

    // users
    pub fn user_make(&self) -> u32 {
        let mut rng = rand::rng();
        let user_id = rng.random();
        self.users.write().unwrap().insert(user_id, User::default());
        user_id
    }
    pub fn user_get(&self, user_id: &u32) -> Option<User> {
        self.users.read().unwrap().get(user_id).cloned()
    }

    pub fn habit_make(&self, user_id: &u32, habit_name: impl Into<String>) -> u32 {
        let h_id = random();
        self.habits.write().unwrap().insert(
            h_id,
            Habit {
                name: habit_name.into(),
            },
        );
        self.users
            .write()
            .unwrap()
            .get_mut(user_id)
            .unwrap()
            .habits
            .insert(h_id);
        h_id
    }
    pub fn habit_get(&self, habit_id: &u32) -> Option<Habit> {
        self.habits.read().unwrap().get(habit_id).cloned()
    }
    pub fn habits_get(&self, user_id: &u32) -> Vec<Habit> {
        self.user_get(user_id)
            .unwrap()
            .habits
            .clone()
            .into_iter()
            .filter_map(|key| self.habit_get(&key))
            .collect::<Vec<Habit>>()
    }

    pub fn habit_ids_get(&self, user_id: &u32) -> Vec<u32> {
        self.user_get(user_id).unwrap().habits.into_iter().collect()
    }
    pub fn habit_delete(&self, habit_id: &u32, user_id: &u32) {
        self.habits.write().unwrap().remove(habit_id);
        self.entries_delete(self.entries_get(habit_id));
        self.users
            .write()
            .unwrap()
            .get_mut(user_id)
            .unwrap()
            .habits
            .remove(habit_id);
    }

    pub fn entries_get(&self, habit_id: &u32) -> Vec<(u32, u32)> {
        self.entries
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|(habit, _)| habit == habit_id)
            .collect()
    }

    pub fn entries_get_finite(&self, habit_id: &u32, days: Vec<u32>) -> Vec<bool> {
        let entries = self.entries_get(habit_id);
        days.into_iter()
            .map(|day| entries.contains(&(*habit_id, day)))
            .collect()
    }

    pub fn entry_make(&self, habit_id: u32, time: u32) {
        self.entries.write().unwrap().insert((habit_id, time));
    }
    pub fn entry_delete(&self, habit_id: u32, time: u32) {
        self.entries.write().unwrap().remove(&(habit_id, time));
    }
    pub fn entries_delete(&self, entries: Vec<(u32, u32)>) {
        entries.into_iter().for_each(|(habit_id, time)| {
            self.entry_delete(habit_id, time);
        });
    }
    pub fn entry_exists(&self, habit_id: u32, time: u32) -> bool {
        self.entries
            .read()
            .unwrap()
            .get(&(habit_id, time))
            .is_some()
    }
    pub fn entry_toggle(&self, habit_id: u32, time: u32) {
        if self.entry_exists(habit_id, time) {
            self.entry_delete(habit_id, time);
        } else {
            self.entry_make(habit_id, time);
        }
    }

    pub fn date_get(&self) -> DateInfo {
        use chrono::Datelike;
        use chrono::Local;
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let days_since_epoch = (now.as_secs() / 86400) as u32;

        let days_since_monday = Local::now().weekday().num_days_from_monday();

        DateInfo {
            days_since_epoch,
            days_since_monday,
            start_of_this_week: days_since_epoch - days_since_monday,
        }
    }

    pub fn habits_get_again(&self, user_id: u32) -> Vec<(u32, Habit)> {
        self.user_get(&user_id)
            .unwrap()
            .habits
            .into_iter()
            .map(|key| (key, self.habit_get(&key).unwrap()))
            .collect()
    }
}
