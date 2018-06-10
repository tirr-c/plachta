#![allow(non_camel_case_types)]

use ::schema::{items_ls, category_map_ls};

make_variant_slice! {
    #[derive(DbEnum, GraphQLEnum, Copy, Clone, PartialEq, Debug)]
    #[DieselType = "Item_category_ls"]
    pub enum ItemCategory [CATEGORY_VARIANTS, CATEGORY_NAMES] {
        Plant,
        MagicGrass,
        Honeycomb,
        Fruit,
        Mushroom,
        Foodstuff,
        Animal,
        Flower,
        Insect,
        Fish,
        Puniball,
        Dragon,
        Scent,
        Water,
        Oil,
        Gas,
        Paper,
        Fuel,
        Powder,
        Wood,
        Thread,
        Cloth,
        Clay,
        Sand,
        Ore,
        Gem,
        Metal,
        MedicineIngredient,
        PoisonIngredient,
        Mysterious,
        Ericsil,  // FIXME
        Activator,
        Anima,
        Counteractive,
        MagicTool,
        Bomb,
        Medicine,
        Sweet,
        Food,
        WeaponMaterial,
        ArmorMaterial,
        WeaponCore,
        WeaponParts,
        MetalWand,
        AlchemicGun,
        Bow,
        Wand,
        Sword,
        Book,
        Armor,
        Accessory,
        CollectingTool,
        Important,
    }
}

make_variant_slice! {
    #[derive(DbEnum, GraphQLEnum, Copy, Clone, PartialEq, Debug)]
    #[DieselType = "Item_type_ls"]
    pub enum ItemType [TYPE_VARIANTS, TYPE_NAMES] {
        Material,
        Disposable,
        Attack,
        Recover,
        Assist,
        Explore,
        Craft,
        Core,
        Sub,
        Weapon,
        Armor,
        Accessory,
        Important,
        Book,
    }
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "items_ls"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub lv: i32,
    pub ty: ItemType,
    pub base_price: Option<i32>,
    pub is_catalyst: bool,
}

#[derive(Insertable)]
#[table_name = "items_ls"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub ty: ItemType,
    pub lv: i32,
    pub base_price: Option<i32>,
    pub is_catalyst: bool,
}

#[derive(AsChangeset)]
#[table_name = "items_ls"]
pub struct UpdateItem<'a> {
    pub name: Option<&'a str>,
    pub ty: Option<ItemType>,
    pub lv: Option<i32>,
    pub base_price: Option<Option<i32>>,
    pub is_catalyst: Option<bool>,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "category_map_ls"]
#[belongs_to(Item, foreign_key = "item_id")]
pub struct CategoryMapItem {
    pub id: i32,
    pub item_id: i32,
    pub category: ItemCategory,
}

#[derive(Insertable)]
#[table_name = "category_map_ls"]
pub struct NewCategoryMapItem {
    pub item_id: i32,
    pub category: ItemCategory,
}

mod graphql_impl {
    use diesel::prelude::*;
    use juniper::FieldResult;
    use super::*;
    use ::graphql::Context;

    graphql_object!(Item: Context |&self| {
        description: "<리디 & 수르의 아틀리에> 아이템 정보를 나타내는 오브젝트입니다."

        field id() -> i32 as "아이템 ID입니다." { self.id }
        field name() -> &str as "아이템 이름입니다." { &self.name }
        field level() -> i32 as "아이템 레벨입니다." { self.lv }
        field item_type() -> ItemType as "아이템 종류입니다." { self.ty }
        field base_price() -> Option<i32> as
        "아이템 기본 가격입니다. null이면 매각 불가능 아이템임을 나타냅니다."
        { self.base_price }
        field is_catalyst() -> bool as
        "촉매 사용 가능 여부를 나타냅니다."
        { self.is_catalyst }

        field categories(&executor) -> FieldResult<Vec<ItemCategory>> as
        "아이템이 속한 카테고리 목록입니다."
        {
            let conn = executor.context().connection_pool().get()?;
            let categories = CategoryMapItem::belonging_to(self)
                .select(category_map_ls::category)
                .load::<ItemCategory>(&conn)?;
            Ok(categories)
        }
    });
}
