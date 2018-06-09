extern crate plachta;
extern crate actix;
extern crate actix_web;
extern crate diesel;
extern crate juniper;
extern crate serde_json;
extern crate failure;
extern crate futures;

use {
    plachta::{
        establish_connection,
        graphql::{
            GraphQlExecutor,
            QueryMessage,
        },
    },
    actix::prelude::*,
    actix_web::{
        App,
        HttpRequest,
        HttpResponse,
        Json,
        server::HttpServer,
    },
    futures::prelude::*,
};

struct State {
    graphql: Addr<Syn, GraphQlExecutor>,
}

fn index(_: HttpRequest<State>) -> &'static str {
    "Hello, world!"
}

fn graphql(req: HttpRequest<State>) -> impl Future<Item=HttpResponse, Error=actix_web::Error> {
    use actix_web::{FromRequest, AsyncResponder};

    let query_future = Json::<QueryMessage>::extract(&req);
    query_future
        .from_err()
        .and_then(move |query| {
            req.state().graphql.send(query.into_inner())
                .from_err()
                .map(|res| {
                    HttpResponse::Ok().json(res)
                })
        })
        .responder()
}

fn main() {
    let sys = actix::System::new("plachta");

    let graphql_addr = SyncArbiter::start(4, || {
        let conn = establish_connection();
        GraphQlExecutor::new(conn)
    });

    HttpServer::new(move || {
        App::with_state(State { graphql: graphql_addr.clone() })
            .resource("/", |r| r.f(index))
            .resource("/graphql", |r| r.post().a(graphql))
    })
        .bind("172.16.0.2:8080").unwrap()
        .start();

    let _ = sys.run();
}
