// use diesel::prelude::*;
// use serde::{Deserialize,Serialize};
// use crate::schema::{users,tv_shows, seasons,episodes,user_videos};

// #[derive(Queryable, Identifiable, Associations,Serialize)]
// #[diesel(belongs_to(User, foreign_key = users_id))]
// #[diesel(belongs_to(TvShows))]
// #[diesel(table_name = user_videos)]
// struct UsersVideos {
//     id: i32,
//     users_id: i32,
//     tv_shows_id: i32,
// }

// #[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,Serialize)]
// #[diesel(belongs_to(Seasons))]
// #[diesel(table_name = episodes)]
// pub struct Episodes {
//     pub id: i32,
//     pub season_number: i32,
//     pub summary: String,
//     pub seasons_id: i32
// }

// #[derive(Queryable, Serialize, Selectable)]
// #[diesel(table_name = users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// #[derive(Clone)]
// pub struct User {
//     pub id: i32,
//     pub user_name: String,
//     pub user_email: String,
//     pub user_password: String,
// }

// #[derive(Insertable)]
// #[diesel(table_name = users)]
// pub struct NewUser<'a> {
//     pub user_name: &'a str,
//     pub user_email: &'a str,
//     pub user_password: &'a str,
// }

// #[derive(Deserialize)]
// pub struct CreateUser {
//     pub user_name: String,
//     pub user_email: String,
//     pub user_password: String,
// }

// #[derive(Deserialize)]
// pub struct UpdateUser {
//     pub user_password: Option<String>,
//     pub user_name: Option<String>
// }

// #[derive(Deserialize)]
// pub struct Login {
//     pub user_email: String,
//     pub user_password: String,
// }

// #[derive(Serialize,Debug)]
// pub struct Response {
//     pub message: String
// }



// #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq,Serialize)]
// #[diesel(table_name = tv_shows)]
// pub struct TvShows {
//     pub id: i32,
//     pub title: String,
//     pub director: String,
//     pub rating: f64,
//     pub summary: String
// }

// #[derive(Insertable)]
// #[diesel(table_name = tv_shows)]
// pub struct NewShow<'a> {
//     pub title: &'a str,
//     pub director: &'a str,
//     pub rating: &'a f64,
//     pub summary: &'a str,
// }

// #[derive(Deserialize)]
// pub struct CreateShow {
//     pub title: String,
//     pub director: String,
//     pub rating: f64,
//     pub summary: String
// }

// #[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq,Serialize)]
// #[diesel(belongs_to(TvShows))]
// #[diesel(table_name = seasons)]
// pub struct Seasons {
//     pub id: i32,
//     pub season_number: i32,
//     pub summary: String,
//     pub tv_shows_id: i32
// }

// #[derive(Insertable)]
// #[diesel(table_name = seasons)]
// pub struct NewSeason<'a> {
//     pub season_number:  &'a i32,
//     pub summary: &'a str,
//     pub tv_shows_id: &'a i32
// }

// #[derive(Deserialize)]
// pub struct CreateSeason {
//     pub season_number: i32,
//     pub summary: String,
// }