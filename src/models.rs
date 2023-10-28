use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct NewPerson {
    #[serde(rename(deserialize = "apelido"))]
    pub nickname: Option<Value>,

    #[serde(rename(deserialize = "nome"))]
    pub name: Option<Value>,

    #[serde(rename(deserialize = "nascimento"))]
    pub birth: Option<Value>,

    #[serde(rename(deserialize = "stack"))]
    pub stack: Option<Value>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct StoredPerson {
    pub id: String,

    pub nickname: String,

    pub name: String,

    pub birth: String,

    pub stack: Option<Vec<String>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct PersonsCount {
    pub count: i64,
}
