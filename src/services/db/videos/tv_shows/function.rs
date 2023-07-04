use diesel::{pg::PgConnection};
use diesel::prelude::*;
use crate::DbError;
use crate::models::models::{CreateShow,NewShow, TvShows};

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