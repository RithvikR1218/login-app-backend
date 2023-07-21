use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::movies;

#[derive(Serialize,Debug)]
pub struct Response {
    pub message: String
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq,Serialize)]
#[diesel(table_name = movies)]
pub struct Movies {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub rating: f64,
    pub summary: String,
    pub duration: f64
}

#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie<'a> {
    pub title: &'a str,
    pub director: &'a str,
    pub rating: &'a f64,
    pub summary: &'a str,
    pub duration: &'a f64
}

#[derive(Deserialize)]
pub struct CreateMovie {
    pub title: String,
    pub director: String,
    pub rating: f64,
    pub summary: String,
    pub duration: f64
}