extern crate plachta;
extern crate diesel;

use self::{
    plachta::{
        establish_connection,
        models::*,
        schema::{
            items_ls,
        },
    },
    diesel::prelude::*,
};

fn main() -> QueryResult<()> {
    let connection_pool = establish_connection();
    let connection = connection_pool.get().unwrap();

    let results =
        items_ls::table
        .limit(5)
        .load::<LSItem>(&connection)?;
    let categories =
        LSCategoryMapItem::belonging_to(&results)
        .load::<LSCategoryMapItem>(&connection)?
        .grouped_by(&results);

    for (item, category_map_items) in results.into_iter().zip(categories) {
        println!("{:?}", item);
        for LSCategoryMapItem { category, .. } in category_map_items {
            println!("{:?}", category);
        }
        println!();
    }

    Ok(())
}
