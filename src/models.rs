#![allow(non_camel_case_types)]

use super::schema::{items_ls, category_map_ls};

#[derive(DbEnum, Copy, Clone, PartialEq, Debug)]
#[DieselType = "Item_category_ls"]
pub enum LSCategory {
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

#[derive(DbEnum, Copy, Clone, PartialEq, Debug)]
#[DieselType = "Item_type_ls"]
pub enum LSType {
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

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "items_ls"]
pub struct LSItem {
    pub id: i32,
    pub name: String,
    pub lv: i32,
    pub ty: LSType,
    pub base_price: Option<i32>,
    pub is_catalyst: bool,
}

#[derive(Insertable)]
#[table_name = "items_ls"]
pub struct LSNewItem<'a> {
    pub name: &'a str,
    pub ty: LSType,
    pub lv: i32,
    pub base_price: Option<i32>,
    pub is_catalyst: bool,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "category_map_ls"]
#[belongs_to(LSItem, foreign_key = "item_id")]
pub struct LSCategoryMapItem {
    pub id: i32,
    pub item_id: i32,
    pub category: LSCategory,
}

#[derive(Insertable)]
#[table_name = "category_map_ls"]
pub struct LSNewCategoryMapItem {
    pub item_id: i32,
    pub category: LSCategory,
}
