use sea_orm::{Database, DatabaseConnection};

pub async fn load() -> DatabaseConnection {
    
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    Database::connect(url)
                                        .await
                                        .expect("Failed to connect to the database")
}
/*
async fn check(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
}
*/