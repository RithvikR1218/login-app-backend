use actix_web::{Responder,web,Error,post};
use crate::services::db::models::videos::tv_shows::episodes::models::CreateEpisode;
use crate::DbPool;
use crate::services::api::videos::tv_shows::seasons::episodes::functions::create_episode;

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
