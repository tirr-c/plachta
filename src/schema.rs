use super::models::*;

table! {
    use diesel::sql_types::*;
    use super::*;

    category_map_ls (id) {
        id -> Int4,
        item_id -> Int4,
        category -> Item_category_ls,
    }
}

table! {
    use diesel::sql_types::*;
    use super::*;

    items_ls (id) {
        id -> Int4,
        name -> Varchar,
        lv -> Int4,
        ty -> Item_type_ls,
        base_price -> Nullable<Int4>,
        is_catalyst -> Bool,
    }
}

joinable!(category_map_ls -> items_ls (item_id));

allow_tables_to_appear_in_same_query!(
    category_map_ls,
    items_ls,
);
