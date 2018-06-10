extern crate plachta;
extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate juniper;
extern crate rand;
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
    rand::prelude::*,
};

embed_migrations!("./migrations");

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

fn main() -> Result<(), failure::Error> {
    dotenv().ok();

    let conn = establish_connection();
    embedded_migrations::run(&conn.get()?)?;

    let auth_key = std::env::var("AUTH_KEY").ok();
    let auth_key = if let Some(auth_key) = auth_key {
        eprintln!("AUTH_KEY set");
        auth_key
    } else {
        eprintln!("Cannot load AUTH_KEY, generating");
        let mut bytes = [0u8; 24];
        thread_rng().fill_bytes(&mut bytes);
        let auth_key = base64::encode(&bytes);
        eprintln!("AUTH_KEY set to {}", auth_key);
        auth_key
    };

    let bind_addr = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
    eprintln!("Bind to {}", bind_addr);

    let sys = actix::System::new("plachta");

    let graphql_addr = SyncArbiter::start(4, move || {
        GraphQlExecutor::new(conn.clone())
    });

    HttpServer::new(move || {
        App::with_state(State { graphql: graphql_addr.clone() })
            .middleware(AuthMiddleware(auth_key.clone()))
            .resource("/graphql", |r| r.post().a(graphql))
    })
        .bind(&bind_addr)?
        .start();

    let _ = sys.run();

    Ok(())
}
