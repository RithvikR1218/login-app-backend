use actix_web::{Responder,HttpResponse,web,Error,get,post,delete};
use crate::services::db::models::videos::tv_shows::episodes::models::CreateEpisode;
use crate::DbPool;
use crate::services::api::videos::tv_shows::seasons::episodes::functions::{
  create_episode,get_all_episodes,get_episodes};

use super::functions::delete_episode;

#[post("/create")]
pub async fn create_new_episode(episode_deets: web:: Path <(String,i32)>,pool: web::Data<DbPool>,info : web::Json<CreateEpisode>) -> Result<impl Responder, Error>{
    let new_season = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_episode(conn,&info,&episode_deets.0,&episode_deets.1)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(new_season))
}

#[get("/show")]
pub async fn get_all_present_episodes(episode_deets: web:: Path <(String,i32)>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_seasons = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_episodes(conn,&episode_deets.0,&episode_deets.1)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(found_seasons))
}

#[get("/show/{name}")]
pub async fn get_some_episodes(episode_deets: web:: Path <(String,i32,String)>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{
      let found_episodes = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_episodes(conn,&episode_deets.0,&episode_deets.1,&episode_deets.2)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(found_episodes))
}

#[delete("/delete/{name}")]
pub async fn delete_particular_episode(episode_deets: web:: Path <(String,i32,String)>,pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
      let results = web::block(move || {
        let conn = &mut pool.get().unwrap();
        delete_episode(conn,&episode_deets.0,&episode_deets.1,&episode_deets.2)
      })
      .await?
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(results)
}