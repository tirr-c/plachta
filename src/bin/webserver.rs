extern crate plachta;
extern crate actix;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
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
        Body,
        HttpRequest,
        HttpResponse,
        Json,
        http::{
            StatusCode,
            header::AUTHORIZATION,
        },
        middleware::{
            Middleware,
            Started,
        },
        server::HttpServer,
    },
    dotenv::dotenv,
    futures::prelude::*,
};

struct AuthMiddleware(String);

impl<S> Middleware<S> for AuthMiddleware {
    fn start(&self, req: &mut HttpRequest<S>) -> actix_web::Result<Started> {
        use actix_web::HttpMessage;

        let headers = req.headers();
        let result = match headers.get(AUTHORIZATION).map(|value| value.to_str()) {
            None => {
                Started::Response(req.response(StatusCode::UNAUTHORIZED, Body::Empty))
            },
            Some(Err(_)) => {
                Started::Response(req.response(StatusCode::FORBIDDEN, Body::Empty))
            },
            Some(Ok(value)) if value != format!("Bearer {}", self.0) => {
                Started::Response(req.response(StatusCode::FORBIDDEN, Body::Empty))
            },
            _ => {
                Started::Done
            }
        };
        Ok(result)
    }
}

struct State {
    graphql: Addr<Syn, GraphQlExecutor>,
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
    dotenv().ok();

    let auth_key = std::env::var("AUTH_KEY").ok();
    if auth_key.is_none() {
        eprintln!("Cannot load AUTH_KEY, serving unauthorized");
    } else {
        eprintln!("AUTH_KEY set");
    }

    let bind_addr = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    eprintln!("Bind to {}", bind_addr);

    let sys = actix::System::new("plachta");

    let graphql_addr = SyncArbiter::start(4, || {
        let conn = establish_connection();
        GraphQlExecutor::new(conn)
    });

    HttpServer::new(move || {
        let mut app = App::with_state(State { graphql: graphql_addr.clone() });
        if let Some(ref value) = auth_key {
            app = app.middleware(AuthMiddleware(value.clone()));
        }
        app.resource("/graphql", |r| r.post().a(graphql))
    })
        .bind(&bind_addr).unwrap()
        .start();

    let _ = sys.run();
}
