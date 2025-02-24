use serde::{Deserialize, Serialize};
use crate::schema::users;
use diesel::prelude::*;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub description: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub name: Option<&'a str>,
    pub description: &'a str,
}
