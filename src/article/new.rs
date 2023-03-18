use std::sync::Arc;

// use ntex::{
//     http::error,
//     web::{
//         self,
//         types::{Json, State},
//         HttpResponse, Responder,
//     },
// };
use ntex::web::{
    // self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{errors::CustomError, models::article::Article, AppState};

// // add new art
// #[web::post("/article")]
// pub async fn new_article(
//     article: Json<Article>,
//     state: State<Arc<AppState>>,
// ) -> Result<impl Responder, CustomError> {
//     let db_pool = &state.db_pool;
//     sqlx::query!(
//         "INSERT INTO articles (title, content) VALUES ($1, $2)",
//         article.title,
//         article.content
//     )
//     .execute(db_pool)
//     .await?;

//     Ok(HttpResponse::Created().body("add arc suc"))
// }

/// add new art 

pub async fn new_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let db_pool = &state.db_pool;
    sqlx::query!(
        "INSERT INTO articles (title, content) VALUES ($1, $2)",
        article.title,
        article.content
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body("add arc suc"))
}