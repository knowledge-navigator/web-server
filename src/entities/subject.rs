use super::course::CourseId;
use serde::{Deserialize, Serialize};

/// Subjects part of a `Course` (Knowledge Navigator).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subject {
    pub id: SubjectId,
    pub course_id: CourseId,
    pub name: String, //
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubjectId(pub String);
