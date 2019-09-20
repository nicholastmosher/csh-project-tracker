#[macro_use]
extern crate diesel;

use std::env;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
