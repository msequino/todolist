use axum::{
    routing::{get},
    Router};
mod endpoints;

pub async fn router() -> Router {
    
    Router::new()
//        .route("/actuator/health", get(endpoints::users::hello()))
        .route("/posts/{id}", 
    get(endpoints::posts::get_post))
        .route("/posts", 
    get(endpoints::posts::get_posts)
                    .post(endpoints::posts::create_post))
        .route("/users/{id}", 
    get(endpoints::users::get_user))
        .route("/users", 
    get(endpoints::users::get_users)
                    .post(endpoints::users::create_user))
        
}
