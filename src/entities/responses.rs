use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{user::UserId, single_chunk::SingleId};

/// Track info on Student review responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: ResponseId,
    pub info_chunk_id: SingleId,
    pub student_id: UserId,
    pub response: ResponseType,
    pub utc_answered: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ResponseType {
    Green,
    Amber,
    Red,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseId(pub String);
