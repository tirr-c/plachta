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
        LSCategoryMapItem,
    },
    schema::{
        category_map_ls,
        items_ls,
    },
};

pub struct Query;
pub struct LS;

graphql_object!(Query: Context |&self| {
    field api_version() -> &str {
        "0.1"
    }

    field ls() -> LS {
        LS
    }
});

graphql_object!(LS: Context |&self| {
    field item(
        &executor,
        id: i32
    ) -> FieldResult<LSItem> {
        let conn = executor.context().conn.get()?;
        Ok(items_ls::table
            .filter(items_ls::id.eq(id))
            .get_result::<LSItem>(&conn)?)
    }

    field items(
        &executor,
        ty: Option<LSType>,
        is_catalyst: Option<bool>
    ) -> FieldResult<Vec<LSItem>> {
        Ok(match (ty, is_catalyst) {
            (Some(ty), Some(is_catalyst)) => {
                let conn = executor.context().conn.get()?;
                items_ls::table
                    .filter(items_ls::ty.eq(ty))
                    .filter(items_ls::is_catalyst.eq(is_catalyst))
                    .load::<LSItem>(&conn)?
            }
            (Some(ty), None) => {
                let conn = executor.context().conn.get()?;
                items_ls::table
                    .filter(items_ls::ty.eq(ty))
                    .load::<LSItem>(&conn)?
            }
            (None, Some(is_catalyst)) => {
                let conn = executor.context().conn.get()?;
                items_ls::table
                    .filter(items_ls::is_catalyst.eq(is_catalyst))
                    .load::<LSItem>(&conn)?
            }
            _ => {
                return Err("invalid query".into());
            }
        })
    }
});
