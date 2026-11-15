use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use sea_orm::{EntityTrait};
use tracing::{info};

use crate::AppState;
use crate::structs::user::{Entity as UserEntity, Model as User};

pub async fn get_user(
    State(state): State<AppState>
    , Path(id): Path<i32>
) -> Result<Json<User>, StatusCode> {
    info!("Fetching single user from the database by id={}", id);
    let user = UserEntity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(user.unwrap()))
}