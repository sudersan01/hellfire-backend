use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type", rename_all = "lowercase", rename = "text")]
pub struct Text {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Marks>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type", content = "content", rename_all = "lowercase")]
pub enum Marks {
    Italic,
    Bold,
}
