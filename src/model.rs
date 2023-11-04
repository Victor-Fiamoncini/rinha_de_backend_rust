use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct StoredPersonModel {
    pub id: Uuid,

    pub nickname: String,

    pub name: String,

    pub birth: String,

    pub stack: Option<Vec<String>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct PersonCountModel {
    pub count: i32,
}
