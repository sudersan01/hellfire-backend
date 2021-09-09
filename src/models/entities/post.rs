use mongodb::bson::{self, Document};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub slug: String,
    pub author: Option<bson::oid::ObjectId>,
    pub content: Option<Document>
}