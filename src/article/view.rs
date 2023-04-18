use ntex::web::types::{Json, Path, State};

use std::sync::Arc;

use crate::{
    errors::CustomError,
    models::article::{Article, ArticlePreview},
    AppState,
};

/// article preview
pub async fn get_articles_preview(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<ArticlePreview>>, CustomError> {
    let db_pool = &state.db_pool;

    // fetch_all will give you a Vec of the rows and give you the ability to map over them
    // fetch_one will give you a single row
    // fetch will give you a stream of rows
    // fetch_optional will give you an Option of a single row
    // fetch_all will give back the connection to the pool
    let articles = sqlx::query!("SELECT id, title, date FROM ARTICLES")
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| ArticlePreview {
            id: i.id as u32,
            title: i.title.clone(),
            date: i.date,
        })
        .collect();

    Ok(Json(articles))
}

/// get one article per id
pub async fn get_article(
    id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Article>, CustomError> {
    let db_pool = &state.db_pool;

    let article = sqlx::query!(
        "SELECT title, content, date FROM articles WHERE id = $1",
        id.0 as i32
    )
    .fetch_one(db_pool)
    .await?;

    let article = Article {
        id: None,
        title: article.title.clone(),
        content: article.content.clone(),
        date: Some(article.date),
    };

    Ok(Json(article))
}

// #[web::get("/articles")]
// pub async fn get_all_articles(
//     state: State<Arc<AppState>>,
// ) -> Result<Json<Vec<Article>>, CustomError> {
//     let db_pool = &state.db_pool;
//     let articles = sqlx::query!("SELECT * FROM articles")
//         .fetch_all(db_pool)
//         .await?
//         .iter()
//         .map(|i| Article {
//             id: Some(i.id as u32),
//             title: i.title.clone(),
//             content: i.content.clone(),
//             date: i.date,
//         })
//         .collect::<Vec<Article>>();

//     Ok(Json(articles))
// }
