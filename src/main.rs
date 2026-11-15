use axum::Router;
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tracing_subscriber;
use tracing::{info, debug};

mod router;
mod database;
mod structs;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv().ok();

    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    
    debug!("Database pool created successfully!");
    let state = AppState {
        db: database::load().await,
    };

    let app: Router = router::router(state);
    let server_url = format!("0.0.0.0:{}", port);
    info!("Server running on http://{}", &server_url);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
