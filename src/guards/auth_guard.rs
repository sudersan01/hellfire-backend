use crate::models::schemas::error_schema::{ErrorMessage, HFError};
use branca::Branca;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    State,
};

pub enum AuthTokenGuard {
    Success { user: String },

    Failure(HFError),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthTokenGuard {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if auth_header.is_none() {
            return Outcome::Success(AuthTokenGuard::Failure(HFError::CustomError(
                ErrorMessage {
                    message: "Authentication Failed, no token found".to_string(),
                    status: Some(Status::BadRequest),
                },
            )));
        }
        let token = auth_header.unwrap();
        let key = request.guard::<&State<&[u8; 32]>>().await;
        if key.is_failure() || key.is_forward() {
            return Outcome::Success(AuthTokenGuard::Failure(HFError::CustomError(
                ErrorMessage {
                    message: "No Auth Header".to_string(),
                    status: Some(Status::BadRequest),
                },
            )));
        }
        let key: &[u8; 32] = key.unwrap();
        let branca = Branca::new(key).unwrap();
        let payload = branca.decode(token, 0);
        if payload.is_err() {
            return Outcome::Success(AuthTokenGuard::Failure(HFError::CustomError(
                ErrorMessage {
                    message: "Invald Token".to_string(),
                    status: Some(Status::BadRequest),
                },
            )));
        }
        let email = String::from_utf8(payload.unwrap()).unwrap();
        return Outcome::Success(AuthTokenGuard::Success { user: email });
    }
}

#[macro_export]
macro_rules! auth_guard {
    ($expr:expr $(,)?) => {
        match $expr {
            AuthTokenGuard::Failure(e) => {
                return Err(e);
            }
            AuthTokenGuard::Success { user } => user,
        }
    };
}
