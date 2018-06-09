use ::{
    PgConnectionPool,
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
use {
    diesel::prelude::*,
    juniper::{
        self,
        FieldResult,
    },
};

mod actor;
mod mutation;
mod query;

pub use self::query::Query;
pub use self::mutation::Mutation;
pub use self::actor::{
    GraphQlExecutor,
    QueryMessage,
    QueryResponse,
};

#[derive(Clone)]
pub struct Context {
    conn: PgConnectionPool,
}
impl juniper::Context for Context {}
impl Context {
    pub fn new(conn: PgConnectionPool) -> Self {
        Context {
            conn
        }
    }

    pub fn connection_pool(&self) -> &PgConnectionPool {
        &self.conn
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
