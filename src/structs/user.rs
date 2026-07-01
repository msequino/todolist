use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
