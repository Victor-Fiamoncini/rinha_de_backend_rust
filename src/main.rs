use actix_web;

mod models;

use crate::models::person::Person;

#[actix_web::get("/health")]
async fn healthcheck() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body("Server is alive")
}

#[actix_web::post("/pessoas")]
async fn store_person(new_person: actix_web::web::Json<Person>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Created()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(healthcheck)
            .service(store_person)
    })
    .bind(("127.0.0.1", 5555))?
    .run()
    .await
}
