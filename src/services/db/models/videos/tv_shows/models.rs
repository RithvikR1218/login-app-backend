use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::{tv_shows};

#[derive(Serialize,Debug)]
pub struct Response {
    pub message: String
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq,Serialize)]
#[diesel(table_name = tv_shows)]
pub struct TvShows {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub rating: f64,
    pub summary: String
}

#[derive(Insertable)]
#[diesel(table_name = tv_shows)]
pub struct NewShow<'a> {
    pub title: &'a str,
    pub director: &'a str,
    pub rating: &'a f64,
    pub summary: &'a str,
}

#[derive(Deserialize)]
pub struct CreateShow {
    pub title: String,
    pub director: String,
    pub rating: f64,
    pub summary: String
}