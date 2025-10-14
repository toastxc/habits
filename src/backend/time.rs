#[cfg(feature = "server")]
use crate::backend::DB;
#[cfg(feature = "server")]
use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[server]
pub async fn date_get() -> Result<DateInfo, ServerFnError> {
    use chrono::Datelike;
    use chrono::Local;
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => now,
        Err(er) => {
            let warning = format!("oh no >: {er}");
            warn!(warning);

            return Err(ServerFnError::new(warning));
        }
    };

    let days_since_epoch = (now.as_secs() / 86400) as u32;

    let days_since_monday = Local::now().weekday().num_days_from_monday();

    Ok(DateInfo {
        days_since_epoch,
        days_since_monday,
        start_of_this_week: days_since_epoch - days_since_monday,
    })
}

#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Debug)]
pub struct DateInfo {
    pub days_since_epoch: u32,
    pub days_since_monday: u32,
    pub start_of_this_week: u32,
}
