use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
    serde::Serialize,
};
use serde_json::json;
use std::io::Cursor;

pub struct HFResponse<T: Serialize> {
    pub status: Option<Status>,
    pub response: T,
    pub error_hint_message: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseSchema<T: Serialize> {
    pub data: T,
    pub status: ServerResponseStatus,
    pub hint: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ServerResponseStatus {
    INFO,
    SUCCESS,
    REDIRECT,
    CLIENTERROR,
    SERVERERROR,
}

impl ServerResponseStatus {
    pub fn new(status: Status) -> ServerResponseStatus {
        return match status.code {
            100..=199 => ServerResponseStatus::INFO,
            200..=299 => ServerResponseStatus::SUCCESS,
            300..=399 => ServerResponseStatus::REDIRECT,
            400..=499 => ServerResponseStatus::CLIENTERROR,
            500..=599 => ServerResponseStatus::SERVERERROR,
            _ => ServerResponseStatus::SERVERERROR,
        };
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for HFResponse<T> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let stat = match self.status {
            Some(val) => val,
            None => Status::Ok,
        };
        let res = ResponseSchema {
            data: self.response,
            status: ServerResponseStatus::new(stat),
            hint: self.error_hint_message,
        };
        let serialized = serde_json::to_string(&res)
            .unwrap_or(json!({"error": "json parsing error"}).to_string());
        Response::build()
            .sized_body(
                serialized.len(),
                Cursor::new(serialized.as_str().to_owned()),
            )
            .status(stat)
            .header(ContentType::JSON)
            .ok()
    }
}
