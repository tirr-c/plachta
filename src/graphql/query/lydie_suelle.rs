use {
    diesel::prelude::*,
    juniper::{
        FieldResult,
    },
};
use ::{
    graphql::Context,
    models::{
        LSCategory,
        LSType,
        LSItem,
    },
    schema::{
        category_map_ls,
        items_ls,
    },
};

pub struct LydieSuelle;

graphql_object!(LydieSuelle: Context |&self| {
    description: "<리디&수르의 아틀리에> 정보를 쿼리할 수 있는 오브젝트입니다."

    field item(
        &executor,
        id: i32
    ) -> FieldResult<LSItem> as
    "특정 ID를 가진 아이템 정보를 가져옵니다."
    {
        let conn = executor.context().connection_pool().get()?;
        Ok(items_ls::table
            .filter(items_ls::id.eq(id))
            .get_result::<LSItem>(&conn)?)
    }

    field items(
        &executor,
        ty: Option<LSType>,
        is_catalyst: Option<bool>,
        category: Option<Vec<LSCategory>>
    ) -> FieldResult<Vec<LSItem>> as
    "조건에 맞는 아이템 정보를 가져옵니다."
    {
        let category = category.and_then(|v| if v.is_empty() { None } else { Some(v) });
        if ty.is_none() && is_catalyst.is_none() && category.is_none() {
            return Err("invalid query".into());
        }

        let conn = executor.context().connection_pool().get()?;
        let mut query = items_ls::table
            .distinct_on(items_ls::id)
            .into_boxed();
        if let Some(ty) = ty {
            query = query.filter(items_ls::ty.eq(ty));
        }
        if let Some(is_catalyst) = is_catalyst {
            query = query.filter(items_ls::is_catalyst.eq(is_catalyst));
        }
        Ok(match category {
            Some(category) => {
                let query = query
                    .inner_join(category_map_ls::table)
                    .select(items_ls::all_columns)
                    .filter(category_map_ls::category.eq_any(category));
                query.load::<LSItem>(&conn)?
            },
            None => {
                query.load::<LSItem>(&conn)?
            },
        })
    }
});
