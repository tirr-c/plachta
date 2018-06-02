#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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
