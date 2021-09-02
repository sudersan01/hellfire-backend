use rocket::{State, serde::json::Json};
use mongodb::Client;

use crate::{DbOptions, auth_guard, guards::auth_guard::AuthTokenGuard, models::{entities::post::Post, schemas::error_schema::HFError}};

#[post("/post/new", data="<post>")]
pub async fn add_post(
    post: Json<Post>,
    opt: &State<DbOptions>,
    user: AuthTokenGuard,
) -> Result<String, HFError> {
    let val = auth_guard!(user);
    let db = Client::with_options(opt.options.clone())?.database("hellfire").collection::<Post>("posts");
    let res = db.insert_one(&post.into_inner(), None).await.unwrap();
    
    Ok(res.inserted_id.to_string())
}
