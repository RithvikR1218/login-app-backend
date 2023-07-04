use crate::services::api::user::endpoints::{
    create_new_user,get_all_present_user,
    get_some_user,update_particular_user,
    delete_particular_user,login_user};
use crate::services::api::videos::tv_shows::endpoints::create_new_show;

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
                            .service(create_new_show);
}
