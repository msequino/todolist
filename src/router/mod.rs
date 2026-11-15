use axum::{
    routing::{get},
    Router};
mod endpoints;

use crate::router::endpoints::posts;
use crate::router::endpoints::users;

use crate::AppState;

pub fn router(state: AppState) -> Router {
    
    Router::new()
        //.route("/actuator/health", get(endpoints::users::hello()))
        .route("/posts/{id}", get(posts::get_post))
        .route("/posts", get(posts::get_all).post(posts::create))
        .route("/users/{id}", get(users::get))
        .route("/users", get(users::get_all).post(users::create))
        //.layer(TraceLayer::new_for_http())
        .with_state(state)
}
