use chrono::prelude::*; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};
use super::{user::UserId, knowledge_nav::KnowledgeNavId};

/// a knowledge navigator course created and managed by teacher accounts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub id: CourseId,
    pub parent_know_nav_id: KnowledgeNavId,
    pub subject: String,
    pub title: String,
    pub description: Option<String>,
    pub utc_created: DateTime<Utc>,
    pub utc_last_updated: DateTime<Utc>,
    /// `Teacher`s responsible for the course
    pub moderators: Vec<UserId>, // only UserType::Teacher
    // /// Questions, info sections and translations that belong to this course
    // pub info_chunks: Option<Vec<ChunkId>>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseId(pub String);