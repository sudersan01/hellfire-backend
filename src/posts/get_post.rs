use mongodb::{Client, bson::{Document, doc}, options::FindOptions};
use rocket::{
    futures::TryStreamExt,
    serde::{Deserialize, Serialize},
    State,
};

use crate::{
    models::{
        schemas::response_schema::HFResponse,
    },
    utils::HFResult,
    DbOptions,
};

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
