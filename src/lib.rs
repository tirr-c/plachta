extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
extern crate failure;
#[macro_use]
extern crate juniper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager,
    },
};

pub mod schema;
#[macro_use]
mod macros;
pub mod models;
pub mod graphql;
pub mod ops;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(
    database_url: impl Into<String>
) -> Result<ConnectionPool, failure::Error> {
    let ret = r2d2::Pool::builder()
        .max_size(4)
        .build(ConnectionManager::new(database_url.into()))?;
    Ok(ret)
}
