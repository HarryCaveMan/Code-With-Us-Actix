use diesel::prelude::*;
use crate::mysql::models::{User,UserSnapshot,MinimalUserSnapshot};
use crate::mysql::DbError;

pub fn get_user(user_id: i32,connection: &mut MysqlConnection) -> Result<Option<User>,DbError> {
    use crate::mysql::schema::Users::id;
    use crate::mysql::schema::Users::dsl::Users;
    let user: Option<User> = Users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .optional()?;
    Ok(user)
}

pub fn get_user_snapshot(user_id: i32,connection: &mut MysqlConnection) -> Result<Option<UserSnapshot>,DbError> {
    use crate::mysql::schema::Users::id;
    use crate::mysql::schema::Users::dsl::Users;
    let user: Option<UserSnapshot> = Users
        .filter(id.eq(user_id))
        .first::<UserSnapshot>(connection)
        .optional()?;
    Ok(user)
}

pub fn get_minimal_user_snapshot(user_id: i32,connection: &mut MysqlConnection) -> Result<Option<MinimalUserSnapshot>,DbError> {
    use crate::mysql::schema::Users::id;
    use crate::mysql::schema::Users::dsl::Users;
    let user: Option<MinimalUserSnapshot> = Users
        .filter(id.eq(user_id))
        .first::<MinimalUserSnapshot>(connection)
        .optional()?;
    Ok(user)
}