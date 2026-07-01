use axum::Extension;
use dotenvy::dotenv;
use tracing_subscriber;
use tracing::{info, debug};

mod router;
mod database;
mod structs;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv().ok();

    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    
    debug!("Database pool created successfully!");

    let app = router::router().await.layer(Extension(database::load().await));
    let server_url = format!("0.0.0.0:{}", port);
    let listener = 
        tokio::net::TcpListener::bind(&server_url).await.unwrap();

    info!("Server running on http://{}", server_url);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}