use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

// Represents a post in the database
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
}
// End represents a post in the database

// Data Transfer Object (DTO) for creating a new post
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
//End Data Transfer Object (DTO) for creating a new post

// Data Transfer Object (DTO) for post response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
}
// End Data Transfer Object (DTO) for post response