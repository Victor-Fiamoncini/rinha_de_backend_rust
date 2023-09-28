use actix_web::{self, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod models;

use crate::models::person::Person;

#[derive(Deserialize)]
struct FetchPersonByTermQueryParams {
    t: String,
}

#[post("/pessoas")]
async fn store_person(_new_person: web::Json<Person>) -> impl Responder {
    HttpResponse::Created()
        .content_type("text/html")
        .body("Person stored")
}

#[get("/pessoas/{id}")]
async fn fetch_person_by_id(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Person by id: {}", path.0))
}

#[get("/pessoas")]
async fn fetch_person_by_term(query: web::Query<FetchPersonByTermQueryParams>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Persons by term: {}", query.t))
}

#[get("/contagem-pessoas")]
async fn count_persons() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("11")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(store_person)
            .service(fetch_person_by_id)
            .service(fetch_person_by_term)
            .service(count_persons)
    })
    .bind(("127.0.0.1", 5555))?
    .run()
    .await
}
