use actix_web::{Responder,HttpResponse,web,Error,post,get,delete};
use crate::services::db::models::videos::movies::models::CreateMovie;
use crate::DbPool;
use crate::services::api::videos::movies::functions::{create_movie, get_all_movies,get_movies,delete_movie};

#[post("/create")]
pub async fn create_new_movie(pool: web::Data<DbPool>,info : web::Json<CreateMovie>) -> Result<impl Responder, Error>{
    let new_movie = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_movie(conn,&info)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(new_movie))
}

#[get("/show")]
pub async fn get_all_present_movies(pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_movies = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_movies(conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(found_movies))
}

#[get("/show/{name}")]
pub async fn get_some_movie(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let details = name.into_inner();
      let found_movie = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_movies(conn,&details)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(found_movie))
}

#[delete("/delete/{name}")]
pub async fn delete_particular_movies(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
  let details = name.into_inner();
      let results = web::block(move || {
        let conn = &mut pool.get().unwrap();
        delete_movie(conn,&details)
      })
      .await?
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(results)
}