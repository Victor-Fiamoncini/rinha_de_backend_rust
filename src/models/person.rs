use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Person {
    pub id: Option<String>,
    pub nickname: String,
    pub name: String,
    pub birth: String,
    pub stack: Option<Vec<String>>,
}
