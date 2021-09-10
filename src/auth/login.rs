use argon2::{Argon2, PasswordHasher};
use branca::Branca;
use mongodb::{bson::doc, Client};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::time::SystemTime;

use crate::utils::HFResult;
use crate::{
    models::{
        entities::user::UserModel,
        schemas::{
            error_schema::{ErrorMessage, HFError},
            response_schema::HFResponse,
        },
    },
    DbOptions,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    user: UserModel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login", data = "<req>")]
pub async fn login(
    req: Json<LoginRequest>,
    opt: &State<DbOptions>,
    key: &State<&[u8; 32]>
) -> HFResult<LoginResponse> {
    let db = Client::with_options(opt.options.clone())?
        .database("hellfire")
        .collection::<UserModel>("users");
    let res = db.find_one(doc! {"name": &req.email}, None).await?;
    if res.is_none() {
        return Err(HFError::CustomError(ErrorMessage {
            message: "Incorrect Email or Password".to_string(),
            status: Some(Status::BadRequest),
            hint: Some("Your Email/Password does not match our database".to_string())
        }));
    }
    let user = res.unwrap();
    let argon2 = Argon2::default();
    let hash = argon2.hash_password_simple(req.password.as_bytes(), &user.salt)?;
    if hash.to_string() != user.password {
        return Err(HFError::CustomError(ErrorMessage {
            message: "Incorrect Email or Password".to_string(),
            status: Some(Status::BadRequest),
            hint: Some("Your Email/Password does not match our database".to_string())
        }));
    }
    let key: &[u8; 32] = key;
    let mut token = Branca::new(key).unwrap();
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH);
    let timestamp = ts.unwrap();
    println!("{:?}", timestamp);
    let ciphertext = token.encode(user.name.as_bytes()).unwrap();


    Ok(HFResponse {
        status: None,
        response: LoginResponse {
            user: UserModel {
                auth_token: Some(ciphertext),
                ..user
            },
        },
        error_hint_message: None,
    })
}
