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

pub struct Query;
pub struct LydieSuelle;

graphql_object!(Query: Context |&self| {
    field api_version() -> &str {
        "0.1"
    }

    field lydie_suelle() -> LydieSuelle {
        LydieSuelle
    }
});

graphql_object!(LydieSuelle: Context |&self| {
    field item(
        &executor,
        id: i32
    ) -> FieldResult<LSItem> {
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
    ) -> FieldResult<Vec<LSItem>> {
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
