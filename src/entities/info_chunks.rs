// a knowledge navigator info section created and managegd by teacher accounts and part of a course.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoChunk {
    pub id: ChunkId,
    pub title: String,
    pub instructions: Option<String>, // custom instructions for this info_chunk
    pub content: String,
    // TODO: Implement saving images and referencing them here.
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkId(pub String);

