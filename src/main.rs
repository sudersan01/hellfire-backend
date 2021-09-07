#[macro_use]
extern crate rocket;

use std::env;

use mongodb::options::ClientOptions;
use rocket::Config;
use utils::cors::CorsFairing;

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
        let opts = match env::var("MONGO_URL") {
            Ok(val) => {
                println!("Variable MONGO_URL found");
                val
            },
            Err(e) => {
                println!("Variable MONGO_URL missing, using default, err is {}", &e);
                "mongodb://localhost:27017/".to_string()
            }
        };
        DbOptions {
            options: ClientOptions::parse(opts)
                .await
                .unwrap(),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let options = DbOptions::new().await;
    let key = b"supersecretkeyyoushouldnotcommit";
    let port: u16 = match env::var("PORT") {
        Ok(val) => {
            println!("Variable PORT found");
            val.parse().unwrap()
        },
        Err(e) => {
            println!("Variable PORT missing, using default, err is {}", &e);
            8080
        }
    };
    let rocket = rocket::build()
        .mount("/auth", auth::get_routes())
        .mount("/", posts::get_routes())
        .manage(options)
        .manage(key)
        .attach(CorsFairing)
        .configure(Config {
            port,
            // address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0),),
            ..Config::debug_default()
        });
    rocket
}
