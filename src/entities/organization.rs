// time conversions for local take place on frontend
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
    pub moderators: Option<Vec<i32>>, // only UserType::Teacher
    pub members: Option<Vec<i32>>,
    // pub knowledge_nav: Option<Vec<KnowledgeNavId>>, // a knowledge navigator (e.g. Knowledge Navigator Y10)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrganizationId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewOrganization {
    pub name: String,
    pub description: Option<String>,
    pub moderators: Option<Vec<i32>>, // only UserType::Teacher
    pub members: Option<Vec<i32>>,
}