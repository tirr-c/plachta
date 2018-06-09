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
pub mod models;
pub mod graphql;

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

pub fn new_item<'a>(
    conn: &PgConnection,
    name: &'a str,
    lv: i32,
    ty: models::LSType,
    base_price: Option<i32>,
    is_catalyst: bool,
    categories: &'a [models::LSCategory],
) -> QueryResult<models::LSItem>
{
    use schema::{items_ls, category_map_ls};

    let new_item = models::LSNewItem {
        name,
        lv,
        ty,
        base_price,
        is_catalyst,
    };

    let item_result: models::LSItem =
        diesel::insert_into(items_ls::table)
        .values(&new_item)
        .get_result(conn)?;
    let item_id = item_result.id;

    let new_categories =
        categories.iter()
        .map(|&cat| models::LSNewCategoryMapItem { item_id, category: cat })
        .collect::<Vec<_>>();
    diesel::insert_into(category_map_ls::table)
        .values(new_categories)
        .execute(conn)?;

    Ok(item_result)
}
