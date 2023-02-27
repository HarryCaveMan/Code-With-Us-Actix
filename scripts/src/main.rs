pub mod models;
pub mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Serialize};
use std::env;
use self::models::Post;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::Posts::dsl::*;

    let connection: &mut MysqlConnection = &mut establish_connection();
    let results = Posts
        .limit(1)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", serde_json::to_string_pretty(&post).unwrap());
    }
}
