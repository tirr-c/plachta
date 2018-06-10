use {
    diesel::{
        self,
        prelude::*,
    },
    juniper::FieldResult,
};
use ::{
    graphql::Context,
    models::{
        LSCategory,
        LSType,
        LSItem,
        LSUpdateItem,
    },
    schema::{
        category_map_ls,
        items_ls,
    },
};

pub struct Mutation;
struct LydieSuelle;

#[derive(GraphQLInputObject)]
#[graphql(description = "새로 추가할 아이템 정보입니다.")]
struct LSNewItem {
    name: String,
    #[graphql(name = "type")]
    ty: LSType,
    level: i32,
    base_price: Option<i32>,
    is_catalyst: bool,
    categories: Vec<LSCategory>,
}

#[derive(GraphQLInputObject)]
struct LSModifyItem {
    name: Option<String>,
    #[graphql(name = "type")]
    ty: Option<LSType>,
    level: Option<i32>,
    tradeable: Option<bool>,
    base_price: Option<i32>,
    is_catalyst: Option<bool>,
}

impl LSModifyItem {
    fn as_changeset(&self) -> Result<LSUpdateItem, &'static str> {
        let base_price = match (self.tradeable, self.base_price) {
            (None, base_price) => base_price.map(Some),
            (Some(true), Some(b)) => Some(Some(b)),
            (Some(false), None) => Some(None),
            _ => return Err("invalid mutation"),
        };
        Ok(LSUpdateItem {
            name: self.name.as_ref().map(|x| x.as_ref()),
            ty: self.ty,
            lv: self.level,
            base_price,
            is_catalyst: self.is_catalyst,
        })
    }
}

#[derive(GraphQLEnum, Copy, Clone, PartialEq, Eq, Debug)]
enum ModifyCategoryOp {
    Add,
    Remove,
}

#[derive(GraphQLInputObject)]
struct LSModifyCategory {
    category: LSCategory,
    op: ModifyCategoryOp,
}

graphql_object!(Mutation: Context |&self| {
    field lydie_suelle() -> LydieSuelle as
    "<리디&수르의 아틀리에> 정보를 변경합니다."
    {
        LydieSuelle
    }
});

graphql_object!(LydieSuelle: Context as "LydieSuelleMut" |&self| {
    description: "<리디&수르의 아틀리에> 정보를 수정할 수 있는 오브젝트입니다."

    field create_item(&executor, new_item: LSNewItem) -> FieldResult<LSItem> as
    "새 아이템을 만듭니다."
    {
        let conn = executor.context().connection_pool().get()?;
        let result = ::new_item(
            &conn,
            &new_item.name,
            new_item.level,
            new_item.ty,
            new_item.base_price,
            new_item.is_catalyst,
            &new_item.categories
        )?;
        Ok(result)
    }

    field modify_item_data(&executor, id: i32, item: LSModifyItem) -> FieldResult<LSItem> as
    "기존에 있던 아이템의 정보를 수정합니다."
    {
        let conn = executor.context().connection_pool().get()?;
        let changeset = item.as_changeset()?;
        let ret = diesel::update(items_ls::table.filter(items_ls::id.eq(id)))
            .set(changeset)
            .get_result::<LSItem>(&conn)?;
        Ok(ret)
    }

    field modify_item_category(
        &executor,
        id: i32,
        categories: Vec<LSModifyCategory>
    ) -> FieldResult<LSItem> as
    "기존에 있던 아이템의 카테고리를 수정합니다."
    {
        let conn = executor.context().connection_pool().get()?;
        let (adds, removes): (Vec<_>, Vec<_>) =
                              categories
                              .into_iter()
                              .partition(|x| x.op == ModifyCategoryOp::Add);
        let removes: Vec<_> = removes.into_iter().map(|x| x.category).collect();
        diesel::delete(
            category_map_ls::table.filter(
                category_map_ls::item_id.eq(id).and(
                    category_map_ls::category.eq_any(removes)
                )
            )
        ).execute(&conn)?;
        let adds: Vec<_> = adds
            .into_iter()
            .map(
                move |x| (
                    category_map_ls::item_id.eq(id),
                    category_map_ls::category.eq(x.category)
                )
            )
            .collect();
        diesel::insert_into(category_map_ls::table)
            .values(&adds)
            .execute(&conn)?;
        let ret = items_ls::table
            .filter(items_ls::id.eq(id))
            .get_result(&conn)?;
        Ok(ret)
    }

    field set_item_category(
        &executor,
        id: i32,
        categories: Vec<LSCategory>
    ) -> FieldResult<LSItem> as
    "기존에 있던 아이템의 카테고리를 새로 설정합니다."
    {
        let conn = executor.context().connection_pool().get()?;
        diesel::delete(
            category_map_ls::table.filter(
                category_map_ls::item_id.eq(id)
            )
        ).execute(&conn)?;
        let adds: Vec<_> = categories
            .into_iter()
            .map(
                move |category| (
                    category_map_ls::item_id.eq(id),
                    category_map_ls::category.eq(category)
                )
            )
            .collect();
        diesel::insert_into(category_map_ls::table)
            .values(&adds)
            .execute(&conn)?;
        let ret = items_ls::table
            .filter(items_ls::id.eq(id))
            .get_result(&conn)?;
        Ok(ret)
    }
});
