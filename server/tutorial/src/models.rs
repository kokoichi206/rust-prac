use serde::{Deserialize, Serialize};

use super::schema::*;

// 自分で model の型を定義しないといけない！？
// https://gist.github.com/steveh/7c7145409a5eed6b698ee8b609b6d1fc

// Tera に渡すために Serialize, Deserialize が必要。
#[derive(Queryable, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub url_name: String,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub url_name: String,
    pub image_url: String,
}

#[derive(Insertable)]
#[table_name = "technologies"]
pub struct NewTechnology<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub image_url: &'a str,
}

#[derive(Queryable)]
pub struct Adopt {
    pub id: i64,
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "adopts"]
pub struct NewAdopt {
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}
