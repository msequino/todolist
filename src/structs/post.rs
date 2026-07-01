use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
