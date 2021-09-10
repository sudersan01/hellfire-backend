use mongodb::bson::{self, Document};
use rocket::serde::{Deserialize, Serialize};

use super::user::PublicUser;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub slug: String,
    #[serde(skip_deserializing)]
    pub author: Option<PublicUser>,
    pub content: Option<Document>,
    pub title: String
}