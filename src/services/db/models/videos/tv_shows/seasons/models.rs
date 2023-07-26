use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::seasons;
use crate::services::db::models::videos::tv_shows::models::TvShows;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,Serialize)]
#[diesel(belongs_to(TvShows))]
#[diesel(table_name = seasons)]
pub struct Seasons {
    pub id: i32,
    pub season_number: i32,
    pub summary: String,
    pub tv_shows_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = seasons)]
pub struct NewSeason<'a> {
    pub season_number:  &'a i32,
    pub summary: &'a str,
    pub tv_shows_id: &'a i32
}

#[derive(Deserialize)]
pub struct CreateSeason {
    pub season_number: i32,
    pub summary: String,
}