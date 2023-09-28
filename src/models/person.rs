use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: Option<String>,
    pub nickname: String,
    pub name: String,
    pub birth: String,
    pub stack: Option<Vec<String>>,
}
