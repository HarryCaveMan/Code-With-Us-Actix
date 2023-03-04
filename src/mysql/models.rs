use diesel::prelude::*;
use serde::{Serialize};
use super::schema::*;

#[derive(Serialize, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = Users)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: String,
    pub picture: Option<String>,
    pub postCount: Option<i32>,
    pub reputation: Option<i32>,
    pub isModerator: Option<bool>,
    pub reports: Option<String>,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime
}

#[derive(Serialize, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = Users)]
pub struct UserSnapshot {
    pub id: i32,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(skip_serializing)]
    pub email: String,
    pub picture: Option<String>,
    pub postCount: Option<i32>,
    pub reputation: Option<i32>,
    pub isModerator: Option<bool>,
    #[serde(skip_serializing)]
    pub reports: Option<String>,
    pub createdAt: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub updatedAt: chrono::NaiveDateTime
}

#[derive(Serialize, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = Users)]
pub struct MinimalUserSnapshot {
    pub id: i32,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub picture: Option<String>,
    pub postCount: Option<i32>,
    pub reputation: Option<i32>,
    #[serde(skip_serializing)]
    pub isModerator: Option<bool>,
    #[serde(skip_serializing)]
    pub reports: Option<String>,
    pub createdAt: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub updatedAt: chrono::NaiveDateTime
}

#[derive(Serialize, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = Subforums)]
pub struct Subforum {
    pub id: i32,
    pub category: Option<String>,
    pub threadCount: Option<i32>,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime
}

#[derive(Serialize, Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[diesel(belongs_to(User,foreign_key=UserId))]
#[diesel(belongs_to(Subforum,foreign_key=SubforumId))]
#[diesel(table_name = Threads)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub postCount: Option<i32>,
    pub reputation: i32,
    pub createdAt: chrono::NaiveDateTime,
    pub updatedAt: chrono::NaiveDateTime,
    pub SubforumId: Option<i32>,
    pub UserId: Option<i32>
}

#[derive(Serialize, Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
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