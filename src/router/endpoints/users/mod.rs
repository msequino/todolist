pub mod get;
pub mod get_all;
pub mod create;

pub use crate::router::endpoints::users::get::get_user as get;
pub use crate::router::endpoints::users::get_all::get_users as get_all;
pub use crate::router::endpoints::users::create::create_user as create;