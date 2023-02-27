use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use crate::schema::*;

#[derive(Serialize,Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = Users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime,
    pub username: Option<String>,
    pub password: Option<String>,
    pub picture: Option<String>,
    pub postCount: Option<i32>,
    pub reputation: Option<i32>,
    pub isModerator: Option<bool>,
    pub reports: Option<String>,
}

#[derive(Serialize,Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = Subforums)]
pub struct Subforum {
    pub id: i32,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime,
    pub category: Option<String>,
    pub threadCount: Option<i32>
}

#[derive(Serialize,Identifiable, Queryable, PartialEq, Debug)]
#[diesel(belongs_to(User,foreign_key=UserId))]
#[diesel(belongs_to(Subforum,foreign_key=SubforumId))]
#[diesel(table_name = Threads)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub reputation: i32,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime,
    pub postCount: Option<i32>,
    pub SubforumId: Option<i32>,
    pub UserId: Option<i32>
}

#[derive(Serialize,Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User,foreign_key=UserId))]
#[diesel(belongs_to(Thread,foreign_key=ThreadId))]
#[diesel(table_name = Posts)]
pub struct Post {
    pub id: i32,
    pub text: Option<String>,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime,
    pub UserId: Option<i32>,
    pub ThreadId: Option<i32>
}