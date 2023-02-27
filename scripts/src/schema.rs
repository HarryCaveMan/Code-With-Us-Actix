// @generated automatically by Diesel CLI.

diesel::table! {
    Posts (id) {
        id -> Integer,
        text -> Nullable<Text>,
        createdAt -> Datetime,
        updatedAt -> Datetime,
        UserId -> Nullable<Integer>,
        ThreadId -> Nullable<Integer>,
    }
}

diesel::table! {
    Subforums (id) {
        id -> Integer,
        category -> Nullable<Varchar>,
        threadCount -> Nullable<Integer>,
        createdAt -> Datetime,
        updatedAt -> Datetime,
    }
}

diesel::table! {
    Threads (id) {
        id -> Integer,
        title -> Varchar,
        postCount -> Nullable<Integer>,
        reputation -> Integer,
        createdAt -> Datetime,
        updatedAt -> Datetime,
        SubforumId -> Nullable<Integer>,
        UserId -> Nullable<Integer>,
    }
}

diesel::table! {
    Users (id) {
        id -> Integer,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        email -> Varchar,
        picture -> Nullable<Varchar>,
        postCount -> Nullable<Integer>,
        reputation -> Nullable<Integer>,
        isModerator -> Nullable<Bool>,
        reports -> Nullable<Text>,
        createdAt -> Datetime,
        updatedAt -> Datetime,
    }
}

diesel::joinable!(Posts -> Threads (ThreadId));
diesel::joinable!(Posts -> Users (UserId));
diesel::joinable!(Threads -> Subforums (SubforumId));
diesel::joinable!(Threads -> Users (UserId));

diesel::allow_tables_to_appear_in_same_query!(
    Posts,
    Subforums,
    Threads,
    Users,
);
