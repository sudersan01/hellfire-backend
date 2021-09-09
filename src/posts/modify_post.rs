use mongodb::{bson::doc, Client};
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    models::{
        entities::post::Post,
        schemas::{
            error_schema::{ErrorMessage, HFError},
            response_schema::HFResponse,
        },
    },
    utils::HFResult,
    DbOptions,
};

#[post("/post/<slug>", data = "<post>")]
pub async fn modify_post(
    slug: String,
    post: Json<Post>,
    opts: &State<DbOptions>,
) -> HFResult<String> {
    let db = Client::with_options(opts.options.clone())?
        .database("hellfire")
        .collection::<Post>("posts");
    let res = db
        .find_one_and_replace(doc! { "slug": &slug }, post.into_inner(), None)
        .await?;

    if res.is_none() {
        return Err(HFError::CustomError(ErrorMessage {
            message: format!("Post {} not found", &slug),
            status: Some(Status::NotFound),
            hint: Some("The requested post is not found. Are you sure it exists?".to_string())
        }));
    }

    Ok(HFResponse {
        status: None,
        error_hint_message: None,
        response: "Success".to_string(),
    })
}
