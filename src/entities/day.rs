use chrono::prelude::*; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};

/// Day for a knowledge navigator `Course` created and managed by teacher accounts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    pub id: DayId,
    pub day: WeekDay,
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DayId(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
