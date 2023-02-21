use serde::{Deserialize, Serialize};
use super::{course::CourseId, day::DayId};

/// schedule for a knowledge navigator `Course` created and managed by teacher accounts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub id: ScheduleId,
    pub course_id: CourseId,
    pub weeks: Vec<DayId>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub String);