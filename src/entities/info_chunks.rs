// a knowledge navigator info section created and managed by teacher accounts and part of a course.rs
use serde::{Deserialize, Serialize};

use super::course::CourseId;

/// Sections of questions, info or translations that `Student` must revise.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoChunk {
    pub id: ChunkId,
    pub parent_course_id: CourseId,
    pub title: String,
    /// custom instructions for this info_chunk
    pub instructions: Option<String>,
    // /// collection of information part of this info chunk
    // pub chunks: Vec<SingleId>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkId(pub String);
