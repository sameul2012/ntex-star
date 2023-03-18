use serde::{Deserialize, Serialize};

/// article detail info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
}


/// preview article, only title and date
#[derive(Debug, Clone, Serialize)]
pub struct ArticlePreview {
    pub id: u32,
    pub title: String,
    pub date: chrono::NaiveDate,
}
