use std::env;

use actix_web::{self, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod handlers;
mod models;

use handlers::{count_persons, fetch_person_by_id, fetch_person_by_term, store_person};

pub struct AppState {
    pub database_pool: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_port = env::var("API_PORT").expect("API_PORT env must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env must be set");

    let postgres_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Error to connect on Postgres database");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let app_state = AppState {
            database_pool: postgres_pool.clone(),
        };

        App::new()
            .app_data(Data::new(app_state))
            .service(store_person::store_person)
            .service(fetch_person_by_id::fetch_person_by_id)
            .service(fetch_person_by_term::fetch_person_by_term)
            .service(count_persons::count_persons)
    })
    .bind(format!("127.0.0.1:{api_port}"))?
    .run()
    .await
}
