use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FetchPersonByTermParams {
    pub _t: String,
}

#[get("/pessoas")]
async fn fetch_person_by_term(_query: web::Query<FetchPersonByTermParams>) -> impl Responder {
    HttpResponse::Ok()
}
