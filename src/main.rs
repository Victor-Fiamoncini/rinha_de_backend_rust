use std::env;

use actix_web::{self, web::Data, App, HttpServer};
use database::Database;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod database;
mod dto;
mod handlers;
mod model;

use handlers::{count_persons, fetch_person_by_id, fetch_person_by_term, store_person};

pub struct AppState {
    database: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_port = env::var("API_PORT").expect("API_PORT env must be set");
    let database_pool = env::var("DATABASE_POOL").expect("DATABASE_POOL env must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env must be set");

    let postgres_pool = PgPoolOptions::new()
        .max_connections(database_pool.parse::<u32>().unwrap())
        .connect(&database_url)
        .await
        .expect("Error to connect on Postgres database");

    println!(
        "{}",
        format!("☕ Server started successfully on port {api_port}, with database pool of {database_pool}")
    );

    HttpServer::new(move || {
        let app_state = AppState {
            database: Database::new(postgres_pool.clone()),
        };

        App::new()
            .app_data(Data::new(app_state))
            .service(store_person::store_person)
            .service(fetch_person_by_id::fetch_person_by_id)
            .service(fetch_person_by_term::fetch_person_by_term)
            .service(count_persons::count_persons)
    })
    .bind(format!("0.0.0.0:{api_port}"))?
    .run()
    .await
}
