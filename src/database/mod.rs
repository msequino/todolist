use sqlx::postgres::PgPoolOptions;

pub async fn load() -> sqlx::postgres::PgPool {
    
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    return PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("Failed to create database pool");

}   