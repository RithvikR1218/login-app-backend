use diesel::{pg::PgConnection};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    //Reading from env
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    //Creatinf connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    return pool;
}