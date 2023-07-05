use actix_web::{Responder,HttpResponse,web,Error,post,get,delete};
use crate::services::db::models::videos::tv_shows::models::{ CreateShow};
use crate::DbPool;
use crate::services::api::videos::tv_shows::functions::{create_show, get_all_shows,get_shows,delete_show};

#[post("/create")]
pub async fn create_new_show(pool: web::Data<DbPool>,info : web::Json<CreateShow>) -> Result<impl Responder, Error>{
    let new_show = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_show(conn,&info)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(new_show))
}

#[get("/show")]
pub async fn get_all_present_shows(pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_shows = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_shows(conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(found_shows))
}

#[get("/show/{name}")]
pub async fn get_some_show(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let details = name.into_inner();
      let found_show = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_shows(conn,&details)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(found_show))
}

#[delete("/delete/{name}")]
pub async fn delete_particular_show(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
  let details = name.into_inner();
      let results = web::block(move || {
        let conn = &mut pool.get().unwrap();
        delete_show(conn,&details)
      })
      .await?
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(results)
}