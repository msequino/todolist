use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{EntityTrait};
use tracing::{info};

use crate::{AppState, structs::user::{Entity as UserEntity, Model as User}};

pub async fn get_users(
    State(state): State<AppState>
) -> Result<Json<Vec<User>>, StatusCode> {
    info!("Fetching all users from the database");
    let users = UserEntity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}
