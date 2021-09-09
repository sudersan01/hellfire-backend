use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
    serde::{Deserialize, Serialize},
};
use serde_json::json;
use std::io::Cursor;

use crate::models::schemas::response_schema::{ResponseSchema, ServerResponseStatus};

#[derive(Debug)]
pub enum HFError {
    MongoError(mongodb::error::Error),
    HashError(argon2::password_hash::Error),
    CustomError(ErrorMessage),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorMessage {
    pub message: String,
    #[serde(skip)]
    pub status: Option<Status>,
    #[serde(skip)]
    pub hint: Option<String>,
}

impl<'r> Responder<'r, 'static> for HFError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mes = match self {
            HFError::CustomError(val) => val,
            HFError::MongoError(err) => {
                println!("   >> {:?}", &err);
                ErrorMessage {
                    message: "Internal Server Error".to_string(),
                    status: None,
                    hint: Some(
                        "An Unknown Error occurred, please report it to proper people".to_string(),
                    ),
                }
            },
            HFError::HashError(err) => {
                println!("   >> {:?}", &err);
                ErrorMessage {
                    message: "Internal Server Error".to_string(),
                    status: None,
                    hint: Some(
                        "An Unknown Error occurred, please report it to proper people".to_string(),
                    ),
                }
            },
        };
        let serialized = serde_json::to_string(&ResponseSchema {
            status: ServerResponseStatus::new(mes.status.unwrap_or(Status::InternalServerError)),
            data: &mes,
            hint: mes.hint.clone(),
        })
        .unwrap_or(json!({"error": "json parsing error"}).to_string());
        Response::build()
            .sized_body(
                serialized.len(),
                Cursor::new(serialized.as_str().to_owned()),
            )
            .status(mes.status.unwrap_or(Status::InternalServerError))
            .header(ContentType::JSON)
            .ok()
    }
}

impl From<mongodb::error::Error> for HFError {
    fn from(err: mongodb::error::Error) -> Self {
        HFError::MongoError(err)
    }
}

impl From<argon2::password_hash::Error> for HFError {
    fn from(err: argon2::password_hash::Error) -> Self {
        HFError::HashError(err)
    }
}
