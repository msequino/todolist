use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{EntityTrait};
use tracing::{info};

use crate::{AppState, structs::post::{Entity as PostEntity, Model as Post}};

pub async fn get_posts(
    State(state): State<AppState>
) -> Result<Json<Vec<Post>>, StatusCode> { 
    info!("Fetching all posts from the database");
    let posts = PostEntity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}
