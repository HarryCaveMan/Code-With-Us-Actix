mod mysql;
use std::fs;
use diesel::prelude::*;
use serde::{Serialize};
use mysql::models::*;
use mysql::util::*;

fn main() {
    use mysql::schema::Users::dsl::*;
    use mysql::schema::Subforums::dsl::*;
    use mysql::schema::Threads::dsl::*;
    use mysql::schema::Posts::dsl::*;
    let connection: &mut MysqlConnection = &mut establish_connection();
    let users = Users
        .load::<User>(connection)
        .expect("Error loading users");
    let subforums = Subforums
        .load::<Subforum>(connection)
        .expect("Error loading subforums");
    let threads = Threads
    .load::<Thread>(connection)
    .expect("Error loading threads");
    let posts = Posts
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Saving {} users...", users.len());
    fs::write("migrations/dumps/users.json",serde_json::to_string_pretty(&users).unwrap());
    println!("{} users saved!", users.len());
    println!("Saving {} subforums...", subforums.len());
    fs::write("migrations/dumps/subforums.json",serde_json::to_string_pretty(&subforums).unwrap());
    println!("{} subforums saved!", subforums.len());
    println!("Saving {} threads...", threads.len());
    fs::write("migrations/dumps/threads.json",serde_json::to_string_pretty(&threads).unwrap());
    println!("{} subforums saved!", threads.len());
    println!("Saving {} posts...", posts.len());
    fs::write("migrations/dumps/posts.json",serde_json::to_string_pretty(&posts).unwrap());
    println!("{} posts saved!", posts.len());
}
