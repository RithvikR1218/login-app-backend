use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::DbError;
use crate::services::db::models::videos::tv_shows::episodes::models::{CreateEpisode,NewEpisode, Episodes};
use crate::services::api::videos::tv_shows::seasons::functions::get_seasons;

pub fn create_episode(conn: &mut PgConnection, info: &CreateEpisode, show_name: &str,season_num: &i32) -> Result<Episodes,DbError> {
    use crate::schema::episodes;

    let season = get_seasons(conn,show_name,season_num)?;
    let new_episode = NewEpisode { 
        episode_name: &info.episode_name,
        director: &info.director,
        summary: &info.summary,
        seasons_id: &season.id
    };
    let create = diesel::insert_into(episodes::table)
        .values(&new_episode)
        .returning(Episodes::as_returning())
        .get_result(conn)?;
    Ok(create)
}

// pub fn get_all_seasons(conn: &mut PgConnection,show_name: &str) -> Result<Vec<Seasons>,DbError> {
//     use crate::schema::seasons::dsl::*;

//     let show = get_shows(conn,show_name)?;
//     let all_seasons = seasons.filter(tv_shows_id.eq(show.id)).load::<Seasons>(conn);
//     match all_seasons {
//                 Ok(found_seasons) => {
//                     if found_seasons.is_empty() {
//                         let error_message = format!("No seasons found");
//                         let test: DbError = String::from(error_message).into();
//                         Err(test)
//                     } else {
//                         Ok(found_seasons)
//                     }
//                 },
//                 Err(err) => Err(Box::new(err)),
//             }
// }

// pub fn get_seasons(conn: &mut PgConnection,show_name: &str,number: &i32) -> Result<Seasons,DbError> {
//     use crate::schema::seasons::dsl::*;

//     let show = get_shows(conn,show_name)?;
//     let all_seasons = seasons.filter(tv_shows_id.eq(show.id))
//                                                         .filter(season_number.eq(number))
//                                                         .first::<Seasons>(conn);
//     match all_seasons {
//                 Ok(found_seasons) => Ok(found_seasons),
//                 Err(diesel::result::Error::NotFound) => {
//                                 let error_message = format!("No season found with number: {}", number);
//                                 // let custom_error = Response { message: error_message };
//                                 let test: DbError = String::from(error_message).into();
//                                 Err(test)
//                             }
//                 Err(err) => Err(Box::new(err)),
//             }
// }

// pub fn get_shows(conn: &mut PgConnection,name: &str) -> Result<TvShows,DbError> {
//     use crate::schema::tv_shows::dsl::*;

//     let show_result = tv_shows.filter(title.eq(name)).first::<TvShows>(conn);
//     match show_result {
//         Ok(user) => Ok(user),
//         Err(diesel::result::Error::NotFound) => {
//             let error_message = format!("No users found with email: {}", name);
//             // let custom_error = Response { message: error_message };
//             let test: DbError = String::from(error_message).into();
//             Err(test)
//         }
//         Err(err) => Err(Box::new(err)),
//     }
// }

// pub fn delete_show(conn: &mut PgConnection,name: &str) -> Result<Response,DbError> {
//     use crate::schema::tv_shows::dsl::*;
//     let show = get_shows(conn,name)?;
//     diesel::delete(tv_shows.find(show.id)).execute(conn)?;
//     let message = Response {message: String::from("Delete Successful")};
//     Ok(message)
// }