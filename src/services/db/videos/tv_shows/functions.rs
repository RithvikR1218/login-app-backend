use diesel::{pg::PgConnection};
use diesel::prelude::*;
use crate::DbError;
use crate::models::models::{CreateShow,NewShow, TvShows,Response};

pub fn create_show(conn: &mut PgConnection, info: &CreateShow) -> Result<TvShows,DbError> {
    use crate::schema::tv_shows;
    let new_show = NewShow { 
        title: &info.title,
        director: & info.director,
        rating: &info.rating,
        summary: &info.summary
    };
    let create = diesel::insert_into(tv_shows::table)
        .values(&new_show)
        .returning(TvShows::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_shows(conn: &mut PgConnection) -> Result<Vec<TvShows>,DbError> {
    use crate::schema::tv_shows::dsl::*;
    let shows = tv_shows.load::<TvShows>(conn)?;
    Ok(shows)
}

pub fn get_shows(conn: &mut PgConnection,name: &str) -> Result<TvShows,DbError> {
    use crate::schema::tv_shows::dsl::*;

    let show_result = tv_shows.filter(title.eq(name)).first::<TvShows>(conn);
    match show_result {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => {
            let error_message = format!("No users found with email: {}", name);
            // let custom_error = Response { message: error_message };
            let test: DbError = String::from(error_message).into();
            Err(test)
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub fn delete_show(conn: &mut PgConnection,name: &str) -> Result<Response,DbError> {
    use crate::schema::tv_shows::dsl::*;
    let show = get_shows(conn,name)?;
    diesel::delete(tv_shows.find(show.id)).execute(conn)?;
    let message = Response {message: String::from("Delete Successful")};
    Ok(message)
}