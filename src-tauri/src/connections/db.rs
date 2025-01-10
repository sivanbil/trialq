use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
