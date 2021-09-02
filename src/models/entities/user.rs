use mongodb::bson;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,

    // #[serde(skip_serializing)]
    pub password: String,

    // #[serde(skip_serializing)]
    pub salt: String,
    pub auth_token: Option<String>
}