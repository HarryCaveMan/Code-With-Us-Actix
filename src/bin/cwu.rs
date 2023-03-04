use diesel::prelude::MysqlConnection;
use serde_json::to_string_pretty;
use code_with_us::controllers::*;
use code_with_us::mysql::DbError;
use code_with_us::mysql::models::*;
use code_with_us::mysql::util::{establish_connection};

pub fn main() {
    let connection: &mut MysqlConnection = &mut establish_connection();
    let id: i32 = 1;    
    let user_result: Result<Option<User>,DbError> =  user_controller::get_user(id,connection);
    let user_snapshot_result: Result<Option<UserSnapshot>,DbError> = user_controller::get_user_snapshot(id,connection);
    let minimal_user_snapshot_result: Result<Option<MinimalUserSnapshot>,DbError> =  user_controller::get_minimal_user_snapshot(id,connection);
    match user_result {
        Ok(user) => println!("Found user {}:\n{}",id,to_string_pretty(&user).unwrap()),
        Err(err) => println!("{}",(*err).to_string())
    }
    match user_snapshot_result {
        Ok(user) => println!("Found user snapshot {}:\n{}",id,to_string_pretty(&user).unwrap()),
        Err(err) => println!("{}",(*err).to_string())
    }
    match minimal_user_snapshot_result {
        Ok(user) => println!("Found minimal user snapshot {}:\n{}",id,to_string_pretty(&user).unwrap()),
        Err(err) => println!("{}",(*err).to_string())
    }
}