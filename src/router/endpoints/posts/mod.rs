mod get;
mod get_all;
mod create;

pub use crate::router::endpoints::posts::get::get_post as get_post;
pub use crate::router::endpoints::posts::get_all::get_posts as get_all;
pub use crate::router::endpoints::posts::create::create_post as create;