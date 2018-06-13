use juniper;
use ::ConnectionPool;

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
    conn: ConnectionPool,
}
impl juniper::Context for Context {}
impl Context {
    pub fn new(conn: ConnectionPool) -> Self {
        Context {
            conn,
        }
    }

    pub fn connection_pool(&self) -> &ConnectionPool {
        &self.conn
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
