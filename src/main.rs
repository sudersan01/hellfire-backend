#[macro_use]
extern crate rocket;

use mongodb::options::ClientOptions;

mod auth;
mod models;
mod posts;
mod utils;
mod guards;

#[derive(Debug, Clone)]
pub struct DbOptions {
    pub options: ClientOptions,
}

impl DbOptions {
    async fn new() -> DbOptions {
        DbOptions {
            options: ClientOptions::parse("mongodb://localhost:27017/")
                .await
                .unwrap(),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let options = DbOptions::new().await;
    let key = b"supersecretkeyyoushouldnotcommit";
    rocket::build()
        .mount("/auth", auth::get_routes())
        .mount("/", posts::get_routes())
        .manage(options)
        .manage(key)
}
