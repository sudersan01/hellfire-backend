use rocket::Route;

use crate::{posts::get_post::get_all_posts};

pub mod add_post;
pub mod get_post;

pub fn get_routes() -> Vec<Route> {
    routes![add_post::add_post, get_all_posts]
}