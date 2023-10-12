use std::env;

use actix_web::{self, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod handlers;
mod models;

pub struct AppState {
    pub database_client: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env must be set");
    let postgres_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Error to connect on Postgres database");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let app_state = AppState {
            database_client: postgres_pool.clone(),
        };

        App::new()
            .app_data(Data::new(app_state))
            .service(handlers::store_person)
            .service(handlers::fetch_person_by_id)
            .service(handlers::fetch_person_by_term)
            .service(handlers::count_persons)
    })
    .bind(("127.0.0.1", 5555))?
    .run()
    .await
}
