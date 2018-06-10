use diesel::{
    self,
    prelude::*,
};
use ::{
    models::lydie_suelle::{
        ItemType,
        ItemCategory,
        Item,
        NewItem,
        NewCategoryMapItem,
    },
    schema::{
        category_map_ls,
        items_ls,
    },
};

pub fn new_item<'a>(
    conn: &PgConnection,
    name: &'a str,
    lv: i32,
    ty: ItemType,
    base_price: Option<i32>,
    is_catalyst: bool,
    categories: &'a [ItemCategory],
) -> QueryResult<Item>
{
    let new_item = NewItem {
        name,
        lv,
        ty,
        base_price,
        is_catalyst,
    };

    let item_result =
        diesel::insert_into(items_ls::table)
        .values(&new_item)
        .get_result::<Item>(conn)?;
    let item_id = item_result.id;

    let new_categories =
        categories.iter()
        .map(|&cat| NewCategoryMapItem { item_id, category: cat })
        .collect::<Vec<_>>();
    diesel::insert_into(category_map_ls::table)
        .values(new_categories)
        .execute(conn)?;

    Ok(item_result)
}
