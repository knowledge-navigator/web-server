use serde::{Deserialize, Serialize};

/// A single question, info or translation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleChunk {
    pub id: SingleId,
    pub content: String,
    // TODO: Implement saving images and referencing them here.
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleId(pub String);

