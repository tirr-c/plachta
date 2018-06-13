use {
    actix::{
        dev::{
            MessageResponse,
            ResponseChannel,
        },
        prelude::*,
    },
    juniper::{
        self,
        ExecutionError,
        Variables,
    },
    serde::{
        Serialize,
        Serializer,
    },
    serde_json,
};
use ::{
    ConnectionPool,
    graphql::{
        Context,
        Mutation,
        Schema,
        Query,
    },
};

pub struct GraphQlExecutor {
    schema: Schema,
    ctx: Context,
}

impl GraphQlExecutor {
    pub fn new(conn: ConnectionPool) -> Self {
        GraphQlExecutor {
            schema: juniper::RootNode::new(Query, Mutation),
            ctx: Context::new(conn),
        }
    }
}

impl Actor for GraphQlExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryMessage {
    query: String,
    operation_name: Option<String>,
    variables: Option<Variables>,
}

#[derive(Serialize, Debug)]
struct QueryResult {
    data: juniper::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ExecutionError>>,
}

#[derive(Debug)]
struct QueryError(serde_json::Value);
impl ::std::ops::Deref for QueryError {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct QueryResponse(Result<QueryResult, QueryError>);

impl QueryResponse {
    pub fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }
}

impl<A> MessageResponse<A, QueryMessage> for QueryResponse
where
    A: Actor,
{
    fn handle<R: ResponseChannel<QueryMessage>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Serialize for QueryResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        match self.0 {
            Ok(ref r) => r.serialize(serializer),
            Err(ref e) => {
                let mut s = serializer.serialize_struct("QueryError", 2)?;
                s.serialize_field("data", &None::<juniper::Value>)?;
                s.serialize_field("errors", &[&e.0])?;
                s.end()
            },
        }
    }
}

impl Message for QueryMessage {
    type Result = QueryResponse;
}

impl Handler<QueryMessage> for GraphQlExecutor {
    type Result = QueryResponse;

    fn handle(&mut self, msg: QueryMessage, _: &mut Self::Context) -> Self::Result {
        let query_result = juniper::execute(
            &msg.query,
            msg.operation_name.as_ref().map(|x| x.as_ref()),
            &self.schema,
            &msg.variables.unwrap_or_else(Variables::new),
            &self.ctx
        );
        let result = query_result
            .map_err(|query_error| {
                let x = serde_json::to_value(query_error);
                QueryError(
                    x.unwrap_or_else(|e| {
                        json!({
                            "message": e.to_string(),
                        })
                    })
                )
            })
            .map(|(data, errors)| {
                QueryResult {
                    data,
                    errors: if errors.len() == 0 { None } else { Some(errors) },
                }
            });
        QueryResponse(result)
    }
}
