extern crate plachta;
extern crate diesel;

use {
    plachta::{
        establish_connection,
        new_item,
        models::*,
        schema::{
            category_map_ls,
        },
    },
    diesel::prelude::*,
};

fn main() -> QueryResult<()> {
    let conn = establish_connection();
    let conn = conn.get().unwrap();

    let item = new_item(
        &conn,
        "톤",
        2,
        LSType::Material,
        Some(1),
        false,
        &[LSCategory::MagicGrass, LSCategory::Plant, LSCategory::MedicineIngredient]
    )?;
    println!("{:?}", item);

    let categories =
        LSCategoryMapItem::belonging_to(&item)
        .select(category_map_ls::category)
        .load::<LSCategory>(&conn)?;
    println!("{:?}", categories);

    Ok(())
}
