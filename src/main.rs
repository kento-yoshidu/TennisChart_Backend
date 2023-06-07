use actix_web::{
    get, App,
    web::{Data},
    Responder, HttpResponse, HttpServer
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, database};

mod service;
use service::{fetch_users};

pub struct AppState {
    db: Pool<Postgres>
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(fetch_users)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
