use actix_web::{Responder,HttpResponse,web,Error,post,get};
use crate::models::models::{ CreateSeason};
use crate::DbPool;
use crate::services::db::videos::tv_shows::seasons::functions::{create_season};

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

#[get("/test")]
pub async fn test() -> impl Responder{
        HttpResponse::Ok().body("Hello, Actix Web!")
}

// #[get("/show")]
// pub async fn get_all_present_shows(pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
//       let found_shows = web::block(move || {
//         let conn = &mut pool.get().unwrap();
//         get_all_shows(conn)
//       })
//       .await?
//       .map_err(actix_web::error::ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok().json(found_shows))
// }

// #[get("/show/{name}")]
// pub async fn get_some_show(name: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
//       let details = name.into_inner();
//       let found_show = web::block(move || {
//         let conn = &mut pool.get().unwrap();
//         get_shows(conn,&details)
//       })
//       .await?
//       .map_err(actix_web::error::ErrorInternalServerError)?;
//     Ok(web::Json(found_show))
// }

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