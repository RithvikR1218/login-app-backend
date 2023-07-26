use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::episodes;
use crate::services::db::models::videos::tv_shows::seasons::models::Seasons;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,Serialize)]
#[diesel(belongs_to(Seasons))]
#[diesel(table_name = episodes)]
pub struct Episodes {
    pub id: i32,
    pub episode_name: String,
    pub director: String,
    pub summary: String,
    pub seasons_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = episodes)]
pub struct NewEpisode<'a> {
    pub episode_name:  &'a str,
    pub director:  &'a str,
    pub summary: &'a str,
    pub seasons_id: &'a i32
}

#[derive(Deserialize)]
pub struct CreateEpisode {
    pub episode_name: String,
    pub director: String,
    pub summary: String,
}