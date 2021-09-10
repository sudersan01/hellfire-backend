use rocket::Route;

use crate::posts::{get_post::get_all_posts, modify_post as modify_post_import};

pub mod add_post;
pub mod get_post;
pub mod modify_post;

pub fn get_routes() -> Vec<Route> {
    routes![
        add_post::add_post,
        get_all_posts,
        modify_post_import::modify_post,
        get_post::get_post_detail
    ]
}

