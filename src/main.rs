extern crate juniper;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::Arc;

mod db;
mod graphql;
mod schema;

use crate::db::establish_connection;
use crate::db::PgPool;
use crate::graphql::{create_schema, Context, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let server_url = dotenv!("SERVER_URL");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = establish_connection();
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(status)))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(server_url)?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct Version {
    app: String,
    version: String,
}

async fn status() -> HttpResponse {
    HttpResponse::Ok().json(Version {
        app: "App Running".to_string(),
        version: "0.0.1-alpha".to_string(),
    })
}

async fn graphql(
    pool: web::Data<PgPool>,
    schema: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };

    let data = web::block(move || {
        let res = request.execute_sync(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(data))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source(dotenv!("GRAPH_URL"), None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
