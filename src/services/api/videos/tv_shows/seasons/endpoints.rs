use actix_web::{Responder,HttpResponse,web,Error,post,get};
use crate::services::db::models::videos::tv_shows::seasons::models::CreateSeason;
use crate::DbPool;
use crate::services::api::videos::tv_shows::seasons::functions::{
    create_season, get_all_seasons,get_seasons};

#[post("/create")]
pub async fn create_new_season(show_name: web:: Path <String>,pool: web::Data<DbPool>,info : web::Json<CreateSeason>) -> Result<impl Responder, Error>{
    let new_season = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_season(conn,&info,&show_name.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(new_season))
}

#[get("/show")]
pub async fn get_all_present_seasons(show_name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_seasons = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_seasons(conn,&show_name.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(found_seasons))
}

#[get("/show/{number}")]
pub async fn get_some_seasons(details: web:: Path <(String,i32)>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let (show_name,season_number) = details.into_inner();
      let found_seasons = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_seasons(conn,&show_name,&season_number)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(found_seasons))
}

// #[delete("/delete/{name}")]
// pub async fn delete_particular_show(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
//   let details = name.into_inner();
//       let results = web::block(move || {
//         let conn = &mut pool.get().unwrap();
//         delete_show(conn,&details)
//       })
//       .await?
//       .map(|user| HttpResponse::Ok().json(user))
//       .map_err(actix_web::error::ErrorInternalServerError)?;
//     Ok(results)
// }