use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct NewPerson {
    #[serde(rename(deserialize = "apelido"))]
    pub nickname: String,

    #[serde(rename(deserialize = "nome"))]
    pub name: String,

    #[serde(rename(deserialize = "nascimento"))]
    pub birth: String,

    #[serde(rename(deserialize = "stack"))]
    pub stack: Option<Vec<String>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct StoredPerson {
    pub id: String,
    pub nickname: String,
    pub name: String,
    pub birth: String,
    pub stack: Option<Vec<String>>,
}
