// a knowledge navigator course created and managegd by teacher accounts
use chrono::prelude::*; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};
use super::{user::UserId, info_chunks::ChunkId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub id: CourseId,
    pub subject: String,
    pub year_group: u8, // 7, 8, 9, 10, 11, 12, 13
    pub title: String,
    pub description: String,
    pub utc_created: DateTime<Utc>,
    pub utc_last_updated: DateTime<Utc>,
    pub moderators: Vec<UserId>, // only UserType::Teacher
    pub info_chunks: Option<Vec<ChunkId>>, // 
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseId(pub String);