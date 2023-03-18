use ntex::web::types::{Json, Path, State};
use std::sync::Arc;

use crate::{errors::CustomError, models::article::ArticlePreview, AppState};

/// search article contain keyword
pub async fn search_article(
    keyword: Path<(String,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<ArticlePreview>>, CustomError> {
   let db_pool = &state.db_pool;

   let result = sqlx::query!(
    "SELECT id, title, date FROM articles WHERE title LIKE $1 or CONTENT LIKE $1",
    format!("%{}%", keyword.0)
    )
    .fetch_all(db_pool)
    .await?
    .iter()
    .map(|i| ArticlePreview{
        id: i.id as u32,
        title: i.title.clone(),
        date: i.date,
    })
    .collect::<Vec<ArticlePreview>>();

    if result.is_empty(){
        return  Err(CustomError::NotFound("No articles match condition".into()));
    }

    Ok(Json(result))
}