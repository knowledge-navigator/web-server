use serde::{Deserialize, Serialize};

use super::{info_chunks::ChunkId, user::UserId};

///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: ResponseId,
    pub info_chunk_id: ChunkId,
    pub student: UserId,
    pub response: ResponseType
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ResponseType {
    Green,
    Amber,
    Red
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseId(pub String);

