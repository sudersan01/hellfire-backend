use branca::Branca;
use rocket::{State, serde::{Deserialize, Serialize, json::Json}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CheckRequest {
    token: String
}

#[post("/check", data = "<token>")]
pub async fn check_auth(token: Json<CheckRequest>, key: &State<&[u8; 32]>) -> String {
    let key: &[u8; 32] = key;
    let branca = Branca::new(key).unwrap();
    let email = branca.decode(token.token.as_str(), 0).unwrap();
    let emailstr = String::from_utf8(email).unwrap();
    emailstr
}