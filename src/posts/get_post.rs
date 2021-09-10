use mongodb::{Client, bson::{Document, doc}, options::FindOptions};
use rocket::{State, futures::TryStreamExt, http::Status, serde::{Deserialize, Serialize}};

use crate::{DbOptions, models::{entities::post::Post, schemas::{error_schema::{ErrorMessage, HFError}, response_schema::HFResponse}}, utils::HFResult};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAllPostResponse {
    pub posts: Vec<Document>,
}

#[get("/")]
pub async fn get_all_posts(opts: &State<DbOptions>) -> HFResult<GetAllPostResponse> {
    let db = Client::with_options(opts.options.clone())?
        .database("hellfire")
        .collection::<Document>("posts");

    let mut res = db
        .find(
            None,
            FindOptions::builder()
                .projection(Some(doc! {
                    "content": 0
                }))
                .build(),
        )
        .await?;
    let mut result = Vec::<Document>::new();
    while let Some(mut x) = res.try_next().await? {
        // x.insert("author", "dhukka");
        result.push(x);
    }

    Ok(HFResponse {
        status: None,
        error_hint_message: None,
        response: GetAllPostResponse { posts: result },
    })
}

#[get("/<slug>")]
pub async fn get_post_detail(slug: String, opts: &State<DbOptions>) -> HFResult<Post> {
    let db = Client::with_options(opts.options.clone())?
        .database("hellfire")
        .collection::<Post>("posts");

    let res = db
        .find_one(
            doc! {"slug":slug},
        None,
        )
        .await?;

    if res.is_none() {
        return Err(HFError::CustomError(ErrorMessage{
            message:"Post not found".to_string(),
            status:Some(Status::NotFound),
            hint: Some("Post not found!!!".to_string())
        }))
    }

    Ok(HFResponse {
        status: None,
        error_hint_message: None,
        response: res.unwrap(),
    })
}
