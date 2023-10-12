use actix_web::{get, web, HttpResponse, Responder};

#[get("/pessoas/{id}")]
async fn fetch_person_by_id(_path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok()
}
