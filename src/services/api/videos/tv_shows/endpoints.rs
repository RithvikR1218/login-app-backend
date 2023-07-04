use actix_web::{Responder,web,Error,post};
use crate::models::models::{ CreateShow};
use crate::DbPool;
use crate::services::db::videos::tv_shows::function::{create_show};

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