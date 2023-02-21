use super::{user::UserId};
use chrono::{DateTime, Utc}; // time conversions for local take place on frontend
use serde::{Deserialize, Serialize};

/**
A collection of knowledge navigators, students and teachers, usually representative of a single school
and its subject material.

An `Organization` can be created any `Teacher`, who is automatically also the moderator of said `Organization`.
Moderator teachers can manipulate courses and members of the `Organization` as well as alleviate or remove any
teacher to and from moderator status.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub description: Option<String>,
    pub utc_created: DateTime<Utc>,
    pub moderators: Vec<UserId>, // only UserType::Teacher
    pub members: Vec<UserId>,
    // pub knowledge_nav: Option<Vec<KnowledgeNavId>>, // a knowledge navigator (e.g. Knowledge Navigator Y10)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrganizationId(pub String);
