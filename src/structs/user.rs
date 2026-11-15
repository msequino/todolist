use sea_orm::entity::prelude::*;
use sea_orm::{EnumIter, Related, RelationDef, DeriveRelation};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::post::Entity")]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Data Transfer Object (DTO) for creating a new user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
// End Data Transfer Object (DTO) for creating a new user

// Data Transfer Object (DTO) for user response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}
// End Data Transfer Object (DTO) for user response

