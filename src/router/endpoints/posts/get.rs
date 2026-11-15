use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use sea_orm::{EntityTrait};
use tracing::{info};

use crate::router::AppState;
use crate::structs::post::{Entity as PostEntity, PostResponse};

pub async fn get_post(
    State(state): State<AppState>
    , Path(id): Path<i32>
) -> Result<Json<PostResponse>, StatusCode> { // Result<Json<Option<Post>>, StatusCode> {
    info!("Fetching single post from the database by id={}", id);
    let post = PostEntity::find_by_id(id)
        .one(&state.db)
        .await;

    match post {
        Ok(Some(post)) => Ok(Json(
            PostResponse {
                id: post.id,
                user_id: post.user_id,
                title: post.title,
                body: post.body,
            }
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}