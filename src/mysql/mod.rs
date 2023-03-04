pub mod models;
pub mod schema;
pub mod util;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;