use {
    diesel::prelude::*,
    juniper::FieldResult,
};
use ::{
    graphql::{
        Context,
    },
    models::{
        LSCategory,
        LSType,
        LSItem,
    },
};

pub struct Mutation;
pub struct LydieSuelleMut;

#[derive(GraphQLInputObject)]
struct LSNewItem {
    name: String,
    #[graphql(name = "type")]
    ty: LSType,
    level: i32,
    base_price: Option<i32>,
    is_catalyst: bool,
    categories: Vec<LSCategory>,
}

graphql_object!(Mutation: Context |&self| {
    field lydie_suelle() -> LydieSuelleMut {
        LydieSuelleMut
    }
});

graphql_object!(LydieSuelleMut: Context |&self| {
    field create_item(&executor, new_item: LSNewItem) -> FieldResult<LSItem> {
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
});
