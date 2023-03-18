use std::sync::Arc;

// use ntex::web;

use ntex::web::types::{Json, State};

use crate::{errors::CustomError, models::article::Article, AppState};

/// mod art per id
// #[web::put("/article")]
pub async fn edit_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    // Article ID
    let id = match article.id {
        Some(id) => id,
        None => return Err(CustomError::BadRequest("pls give correct id ".into())),
    };

    sqlx::query!(
        "UPDATE articles SET title = $1, content = $2 where id = $3",
        article.title,
        article.content,
        id as i32,
    )
    .execute(db_pool)
    .await?;

    Ok("mod suc !!!".into())
}
