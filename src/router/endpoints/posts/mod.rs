use axum::extract::Path;
use axum::{Extension, Json, http::StatusCode};
use tracing::{info};

use crate::structs::post::Post;
use crate::structs::post::CreatePost;

pub async fn get_post(
    Extension(pool): Extension<sqlx::postgres::PgPool>
    , Path(id): Path<i32>
) -> Result<Json<Post>, StatusCode> {
    info!("Fetching single post from the database by id{}", id);
    let post = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(post))
}

pub async fn get_posts(
    Extension(pool): Extension<sqlx::postgres::PgPool>
) -> Result<Json<Vec<Post>>, StatusCode> {
    info!("Fetching all posts from the database");
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts))
}

pub async fn create_post(
    Extension(pool): Extension<sqlx::postgres::PgPool>
    , Json(payload): Json<CreatePost>
) -> Result<Json<Post>, StatusCode> {
    info!("Creating post in the database {:#?}", payload);
    let res = sqlx::query_as!(Post
        , "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, user_id, title, body"
        , payload.user_id, payload.title, payload.body)
        .fetch_one(&pool)
        .await;
    match res {
        Ok(post) => {
            info!("Saved post {:#?}", post);
            Ok(Json(post))
        },
        Err(e) => {
            info!("Error saving post: {:#?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}