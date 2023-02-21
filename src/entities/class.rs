use super::{course::CourseId, user::UserId};
use chrono::{DateTime, Utc}; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};

/// A collection of `Student` accounts under watch by a `Teacher`. Primary use involves for `Teacher` dashboard info.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub id: ClassId,
    pub name: String,
    pub form: String,
    pub utc_created: DateTime<Utc>,
    pub utc_modified: DateTime<Utc>,
    pub students: Vec<UserId>, // only UserType::Student
    pub teachers: Vec<UserId>, // only UserType::Teacher
    /// `courses` that belong to said `class`
    pub courses: Option<Vec<CourseId>>, // a knowledge navigator (e.g. Knowledge Navigator Y10)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassId(pub String);
