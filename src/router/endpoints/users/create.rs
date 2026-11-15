use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveValue::Set, EntityTrait};
use tracing::{info};

use crate::{AppState, structs::user::{self, CreateUser, Entity as UserEntity, UserResponse}};

pub async fn create_user(
    State(state): State<AppState>
    , Json(payload): Json<CreateUser>
) -> Result<Json<UserResponse>, StatusCode> {
    info!("Creating user in the database {:#?}", payload);

    let CreateUser {
        username,
        email,
    } = payload;

    let ar = user::ActiveModel {
        username: Set(username.clone()),
        email: Set(email.clone()),
        ..Default::default()
    };
    let res = UserEntity::insert(ar)
        .exec(&state.db)
        .await;

    match res {
        Ok(user) => {
            info!("Saved user {:#?}", user);
            Ok(Json(UserResponse {
                id: user.last_insert_id,
                username,
                email,
            }))
        }
        Err(e) => {
            info!(
                "Error saving user: username={username:?}, email={email:?}, error={e:#?}"
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}