use crate::{DbOptions, models::{
        entities::user::UserModel,
        schemas::response_schema::HFResponse,
    }, utils::HFResult};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use mongodb::Client;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    State,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SignupResponse {
    message: String,
}

#[post("/signup", data = "<user>")]
pub async fn signup(
    user: Json<User>,
    opt: &State<DbOptions>,
) -> HFResult<SignupResponse> {
    let userdb = Client::with_options(opt.options.clone())?
        .database("hellfire")
        .collection::<UserModel>("users");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(
            &(user.password).as_bytes(),
            None,
            Params::default(),
            &salt,
        )?
        .to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    if argon2
        .verify_password(&(user.password).as_bytes(), &parsed_hash)
        .is_err()
    {
        return Ok(HFResponse {
            error_hint_message: Some("Password hashing failed".to_string()),
            status: None,
            response: SignupResponse {
                message: "Server Error".to_string(),
            },
        });
    }
    userdb
        .insert_one(
            UserModel {
                id: None,
                name: user.email.clone(),
                password: password_hash.clone(),
                salt: salt.as_str().to_string(),
                auth_token: None,
            },
            None,
        )
        .await?;
    Ok(HFResponse {
        status: None,
        response: SignupResponse {
            message: "Login Success".to_string(),
        },
        error_hint_message: None,
    })
}
