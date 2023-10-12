use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewPerson {
    pub nickname: String,
    pub name: String,
    pub birth: String,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct StoredPerson {
    pub id: String,
    pub nickname: String,
    pub name: String,
    pub birth: String,
    pub stack: Option<Vec<String>>,
}
