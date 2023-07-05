//Use gives the file path for the functions
use crate::services::api::user::endpoints::{
    //The functions being used with endpoints
    create_new_user,get_all_present_user,
    get_some_user,update_particular_user,
    delete_particular_user,login_user};
    
use crate::services::api::videos::tv_shows::endpoints::{
    create_new_show,get_all_present_shows,get_some_show,
    delete_particular_show};

use crate::services::api::videos::tv_shows::seasons::endpoints::{
    create_new_season,get_all_present_seasons,get_some_seasons};

pub fn user_controller() -> actix_web::Scope{
    return actix_web::web::scope("/user") 
                                        .service(create_new_user)
                                        .service(get_all_present_user)
                                        .service(get_some_user)
                                        .service(update_particular_user)
                                        .service(delete_particular_user)
                                        .service(login_user);
}

pub fn tv_show_controller() -> actix_web::Scope{
    return actix_web::web::scope("/tv_show")
                            .service(create_new_show)
                            .service(get_all_present_shows)
                            .service(get_some_show)
                            .service(delete_particular_show)
                            .service(
                                actix_web::web::scope("/{name}/season")
                                                        .service(create_new_season)
                                                        .service(get_all_present_seasons)
                                                        .service(get_some_seasons)
                            );
}