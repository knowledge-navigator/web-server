use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub role: UserType,
    pub first_name: String,
    pub last_name: String,
    pub form: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserType {
    Student,
    Teacher,
}
