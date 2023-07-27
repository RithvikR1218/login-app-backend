//Use gives the file path for the functions
use crate::services::api::user::endpoints::{
    //The functions being used with endpoints
    create_new_user,get_all_present_user,
    get_some_user,update_particular_user,
    delete_particular_user,login_user};
    
use crate::services::api::videos::tv_shows::endpoints::{
    create_new_show,get_all_present_shows,get_some_show,
    delete_particular_show};

use crate::services::api::videos::movies::endpoints::{
    create_new_movie,get_all_present_movies,get_some_movie,
    delete_particular_movies};


use crate::services::api::videos::tv_shows::seasons::endpoints::{
    create_new_season,get_all_present_seasons,get_some_seasons};

use crate::services::api::videos::tv_shows::seasons::episodes::endpoints::{create_new_episode, get_all_present_episodes, get_some_episodes, delete_particular_episode};

pub fn user_controller() -> actix_web::Scope{
    return actix_web::web::scope("/user") 
                                        .service(create_new_user)
                                        .service(get_all_present_user)
                                        .service(get_some_user)
                                        .service(update_particular_user)
                                        .service(delete_particular_user)
                                        .service(login_user);
}

pub fn movie_controller() -> actix_web::Scope{
    return actix_web::web::scope("/movie") 
                                        .service(create_new_movie)
                                        .service(get_all_present_movies)
                                        .service(get_some_movie)
                                        .service(delete_particular_movies)
}

pub fn tv_show_controller() -> actix_web::Scope{
    return actix_web::web::scope("/tv_show")
                            .service(create_new_show)
                            .service(get_all_present_shows)
                            .service(get_some_show)
                            .service(delete_particular_show)
                            //season controller
                            .service(
                                actix_web::web::scope("/{name}/season")
                                                        .service(create_new_season)
                                                        .service(get_all_present_seasons)
                                                        .service(get_some_seasons)
                                                        .service(
                                                            actix_web::web::scope("/{number}/episode")
                                                            .service(create_new_episode)
                                                            .service(get_all_present_episodes)
                                                            .service(get_some_episodes)
                                                            .service(delete_particular_episode)
                                                        )
                            );
}