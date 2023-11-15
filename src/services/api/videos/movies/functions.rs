use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::DbError;
use crate::services::db::models::videos::movies::models::{CreateMovie,NewMovie, Movies,Response};

pub fn create_movie(conn: &mut PgConnection, info: &CreateMovie) -> Result<Movies,DbError> {
    use crate::schema::movies;
    let new_movie = NewMovie { 
        title: &info.title,
        director: & info.director,
        rating: &info.rating,
        summary: &info.summary,
        duration: &info.duration,
        video_link: &info.video_link,
        picture_link: &info.picture_link,
    };
    let create = diesel::insert_into(movies::table)
        .values(&new_movie)
        .returning(Movies::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_movies(conn: &mut PgConnection) -> Result<Vec<Movies>,DbError> {
    use crate::schema::movies::dsl::*;
    let shows = movies.load::<Movies>(conn)?;
    Ok(shows)
}

pub fn get_movies(conn: &mut PgConnection,name: &str) -> Result<Movies,DbError> {
    use crate::schema::movies::dsl::*;

    let show_result = movies.filter(title.eq(name)).first::<Movies>(conn);
    match show_result {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => {
            let error_message = format!("No shows found with name: {}", name);
            // let custom_error = Response { message: error_message };
            let test: DbError = String::from(error_message).into();
            Err(test)
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub fn delete_movie(conn: &mut PgConnection,name: &str) -> Result<Response,DbError> {
    use crate::schema::movies::dsl::*;
    let show = get_movies(conn,name)?;
    diesel::delete(movies.find(show.id)).execute(conn)?;
    let message = Response {message: String::from("Delete Successful")};
    Ok(message)
}