extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use {
    diesel::{
        prelude::*,
        r2d2::{
            self,
            ConnectionManager,
        },
    },
    dotenv::dotenv,
    std::env,
};

pub mod schema;
#[macro_use]
mod macros;
pub mod models;
pub mod graphql;
pub mod ops;

pub type PgConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnectionPool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set");
    r2d2::Pool::builder()
        .max_size(4)
        .build(ConnectionManager::new(database_url.clone()))
        .expect(&format!("Failed to build db pool ({})", database_url))
}
