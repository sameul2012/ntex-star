use std::sync::Arc;

use ntex::web::{
    types::{Path, State}
};

use crate::{errors::CustomError, AppState};

pub async fn delete_article(
    id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!("DELETE FROM articles WHERE id = $1", id.0 as i32)
    .execute(db_pool)
    .await?;

    Ok("del suc".into())
}