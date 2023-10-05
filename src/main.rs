use actix_web::{self, App, HttpServer};

mod handlers;
mod models;

pub struct AppState {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::store_person)
            .service(handlers::fetch_person_by_id)
            .service(handlers::fetch_person_by_term)
            .service(handlers::count_persons)
    })
    .bind(("127.0.0.1", 5555))?
    .run()
    .await
}
