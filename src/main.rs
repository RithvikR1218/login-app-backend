//To actually use them
mod services { 
    //Adding pub makes it public and we can use the functions in the file
    pub mod db {
        pub mod connection {pub mod db_connection;}
        pub mod user {pub mod functions;}
        pub mod videos {pub mod tv_shows {pub mod function;}}
    } 
    pub mod api {
            pub mod user {pub mod endpoints;}
            pub mod videos {pub mod tv_shows {pub mod endpoints;}}
    }
}
mod models {pub mod models;}
//Using root level file
pub mod schema;
pub mod routes;
//By keeping all these files in main, [Intellisense/Rust sees them]

use actix_web::{web,error,App,middleware, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

//For logging
use env_logger::Env;
//for routes
use crate::routes::{user_controller,tv_show_controller};
//For db pool
use crate::services::db::connection::db_connection::get_connection_pool;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

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

    env_logger::init_from_env(Env::default().default_filter_or("info"));
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
            .app_data(web::Data::new(pool.clone()))
            .service(user_controller())
            .service(tv_show_controller())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}