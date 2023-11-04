use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct NewPersonDTO {
    #[serde(rename(deserialize = "apelido"))]
    pub nickname: Option<Value>,

    #[serde(rename(deserialize = "nome"))]
    pub name: Option<Value>,

    #[serde(rename(deserialize = "nascimento"))]
    pub birth: Option<Value>,

    #[serde(rename(deserialize = "stack"))]
    pub stack: Option<Value>,
}
