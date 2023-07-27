use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::DbError;
use crate::services::db::models::user::models::Response;
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

pub fn get_all_episodes(conn: &mut PgConnection,show_name: &str,season_num: &i32) -> Result<Vec<Episodes>,DbError> {
    use crate::schema::episodes::dsl::*;

    let found_season = get_seasons(conn,show_name,season_num)?;
    let all_episodes = episodes.filter(seasons_id.eq(found_season.id)).load::<Episodes>(conn);
    match all_episodes {
                Ok(found_seasons) => {
                    if found_seasons.is_empty() {
                        let error_message = format!("No seasons found");
                        let test: DbError = String::from(error_message).into();
                        Err(test)
                    } else {
                        Ok(found_seasons)
                    }
                },
                Err(err) => Err(Box::new(err)),
            }
}

pub fn get_episodes(conn: &mut PgConnection,show_name: &str,season_number: &i32,name: &str) -> Result<Episodes,DbError> {
    use crate::schema::episodes::dsl::*;

    let found_season = get_seasons(conn,show_name,season_number)?;
    let all_episodes = episodes.filter(seasons_id.eq(found_season.id))
                                                        .filter(episode_name.eq(name))
                                                        .first::<Episodes>(conn);
    match all_episodes {
                Ok(found_episodes) => Ok(found_episodes),
                Err(diesel::result::Error::NotFound) => {
                                let error_message = format!("No episodes found with name: {}", name);
                                // let custom_error = Response { message: error_message };
                                let test: DbError = String::from(error_message).into();
                                Err(test)
                            }
                Err(err) => Err(Box::new(err)),
            }
}

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

pub fn delete_episode(conn: &mut PgConnection,show_name: &str,season_num: &i32,name: &str) -> Result<Response,DbError> {
    use crate::schema::episodes::dsl::*;
    let found_episode = get_episodes(conn,show_name,season_num,name)?;
    diesel::delete(episodes.find(found_episode.id)).execute(conn)?;
    let message = Response {message: String::from("Delete Successful")};
    Ok(message)
}