use rocket::serde::{Serialize, Deserialize};

use super::text::Text;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub struct Attrs {
    pub level: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub struct Heading {
    pub attrs: Attrs,
    pub content: Vec<Text>
}