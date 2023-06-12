use actix_cors::Cors;
use actix_web::{
    get, App,
    http::header,
    web::{Data},
    Responder, HttpResponse, HttpServer
};
// use dotenv::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, database};

use serde::{Serialize};
mod service;
use service::{test};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}
pub struct AppState {
    db: Pool<Postgres>
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "PORT";
    let default_port = 8888;
    let port = match env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "the port number \"{}\" is invalid. default port will be used.",
                    val
                );
                default_port
            }
        },
        Err(_) => {
            println!(
                "\"{}\" is not defined in environment variables. default port will be used.",
                port_key
            );
            default_port
        }
    };
    /*
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection");
    */

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            // .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(test)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
