use chrono::prelude::*; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};
use super::{user::UserId, course::CourseId};

/// An entire Knowledge Navigator booklet, created and managed by teacher accounts.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeNav {
    pub id: KnowledgeNavId,
    pub year_group: u8, // 7, 8, 9, 10, 11, 12, 13
    pub title: String,
    pub description: Option<String>,
    pub utc_created: DateTime<Utc>,
    pub utc_last_updated: DateTime<Utc>,
    pub moderators: Vec<UserId>, // only UserType::Teacher
    pub courses: Option<Vec<CourseId>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnowledgeNavId(pub String);