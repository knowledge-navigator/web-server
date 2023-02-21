use serde::{Deserialize, Serialize};

use super::info_chunks::ChunkId;

/// A single question, info or translation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleChunk {
    pub id: SingleId,
    pub parent_chunk_id: ChunkId,
    pub content: String,
    // TODO: Implement saving images and referencing them here.
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleId(pub String);

