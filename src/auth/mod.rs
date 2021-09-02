use rocket::Route;

mod signup;
mod login;
mod check_auth;

pub fn get_routes() -> Vec<Route> {
    routes![signup::signup, login::login, check_auth::check_auth]
}