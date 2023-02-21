use super::day::DayId;
use serde::{Deserialize, Serialize};

/// schedule for a knowledge navigator `Course` created and managed by teacher accounts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub id: ScheduleId,
    pub cycle: u8,
    pub weeks: Vec<DayId>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub String);
