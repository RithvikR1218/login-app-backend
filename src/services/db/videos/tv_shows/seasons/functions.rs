use diesel::{pg::PgConnection};
use diesel::prelude::*;
use crate::DbError;
use crate::models::models::{CreateSeason,NewSeason, Seasons};
use crate::services::db::videos::tv_shows::functions::get_shows;

pub fn create_season(conn: &mut PgConnection, info: &CreateSeason, show_name: &str) -> Result<Seasons,DbError> {
    use crate::schema::seasons;

    let show = get_shows(conn,show_name)?;
    let new_season = NewSeason { 
        season_number: &info.season_number,
        summary: &info.summary,
        tv_shows_id: &show.id
    };
    let create = diesel::insert_into(seasons::table)
        .values(&new_season)
        .returning(Seasons::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_seasons(conn: &mut PgConnection,show_name: &str) -> Result<Vec<Seasons>,DbError> {
    use crate::schema::seasons::dsl::*;

    let show = get_shows(conn,show_name)?;
    let all_seasons = seasons.filter(tv_shows_id.eq(show.id)).load::<Seasons>(conn);
    match all_seasons {
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

pub fn get_seasons(conn: &mut PgConnection,show_name: &str,number: &i32) -> Result<Seasons,DbError> {
    use crate::schema::seasons::dsl::*;

    let show = get_shows(conn,show_name)?;
    let all_seasons = seasons.filter(tv_shows_id.eq(show.id))
                                                        .filter(season_number.eq(number))
                                                        .first::<Seasons>(conn);
    match all_seasons {
                Ok(found_seasons) => Ok(found_seasons),
                Err(diesel::result::Error::NotFound) => {
                                let error_message = format!("No season found with number: {}", number);
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

// pub fn delete_show(conn: &mut PgConnection,name: &str) -> Result<Response,DbError> {
//     use crate::schema::tv_shows::dsl::*;
//     let show = get_shows(conn,name)?;
//     diesel::delete(tv_shows.find(show.id)).execute(conn)?;
//     let message = Response {message: String::from("Delete Successful")};
//     Ok(message)
// }