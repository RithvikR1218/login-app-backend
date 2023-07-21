//Module tree to let rust see all the modules
mod services { 
    pub mod db {
        pub mod connection {pub mod db_connection;}
        pub mod models {
            pub mod old_models;
            pub mod user {
                pub mod models;
            }
            pub mod videos {
                pub mod tv_shows {
                    pub mod models;
                    pub mod seasons {
                        pub mod models;
                    }
                }
                pub mod movies {
                    pub mod models;
                }
            }
        }
    } 
    pub mod api {
        pub mod user {
            pub mod endpoints;
            pub mod functions;}
        pub mod videos {
            pub mod tv_shows {
                pub mod endpoints;
                pub mod functions;
                pub mod seasons {
                    pub mod endpoints;
                    pub mod functions;
                }
            }
            pub mod movies {
                pub mod functions;
                pub mod endpoints;
            }
        }
    }
    pub mod routes {
        pub mod routes;
    }
}
pub mod schema;

use actix_web::{web,error,App,middleware, HttpResponse, HttpServer};
use actix_cors::Cors;

//logging
use env_logger::Env;
use crate::services::routes::routes::{user_controller,tv_show_controller,movie_controller};
use crate::services::db::connection::db_connection::get_connection_pool;

//Making a connection pool
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

//Making custom types for later use
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    

    let pool = get_connection_pool();
    println!("Connection to DB Established !\n");

    //To start logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    //Customizing json data
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(json_config)
            //Sending connection pool as a state around the api
            .app_data(web::Data::new(pool.clone()))
            //Controllers being used by server
            .service(user_controller())
            .service(movie_controller())
            .service(tv_show_controller())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}