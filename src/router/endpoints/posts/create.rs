use axum::extract::State;
use axum::{Json, http::StatusCode};
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait};
use tracing::{info};

use crate::AppState;
use crate::structs::post::{self, CreatePost, PostResponse};
use crate::structs::post::{Entity as PostEntity};

pub async fn create_post(
    State(state): State<AppState>
    , Json(payload): Json<CreatePost>
) -> Result<Json<PostResponse>, StatusCode> {
    info!("Creating post in the database {:#?}", payload);

    let CreatePost {
        user_id,
        title,
        body,
    } = payload;

    let model = post::ActiveModel {
        user_id: Set(Some(user_id)),
        title: Set(title.clone()),
        body: Set(body.clone()),
        ..Default::default()
    };
    let res = PostEntity::insert(model)
        .exec(&state.db)
        .await;

    match res {
        Ok(post) => {
            info!("Saved post {:#?}", post);
            Ok(Json(PostResponse {
                id: post.last_insert_id,
                user_id: Some(user_id),
                title,
                body,
            }))
        }
        Err(e) => {
            info!(
                "Error saving post: user_id={user_id}, title={title:?}, body={body:?}, error={e:#?}"
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn fail_test_create_post() {
        
    }
}