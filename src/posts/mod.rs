use rocket::Route;

pub mod add_post;

pub fn get_routes() -> Vec<Route> {
    routes![add_post::add_post]
}