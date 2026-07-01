use axum::extract::Path;
use axum::{Extension, Json, http::StatusCode};
use tracing::{info};

use crate::structs::user::User;
use crate::structs::user::CreateUser;

pub async fn get_user(
    Extension(pool): Extension<sqlx::postgres::PgPool>
    , Path(id): Path<i32>
) -> Result<Json<User>, StatusCode> {
    info!("Fetching single user from the database by id {}", id);
    let user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

pub async fn get_users(
    Extension(pool): Extension<sqlx::postgres::PgPool>
) -> Result<Json<Vec<User>>, StatusCode> {
    info!("Fetching all users from the database");
    let users = sqlx::query_as!(User, "SELECT id, username, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

pub async fn create_user(
    Extension(pool): Extension<sqlx::postgres::PgPool>
    , Json(payload): Json<CreateUser>
) -> Result<Json<User>, StatusCode> {
    info!("Creating user in the database {:#?}", payload);
    let res = sqlx::query_as!(User
        , "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email"
        , payload.username, payload.email)
        .fetch_one(&pool)
        .await;
    match res {
        Ok(user) => {
            info!("Saved user {:#?}", user);
            Ok(Json(user))
        },
        Err(e) => {
            info!("Error saving user: {:#?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}