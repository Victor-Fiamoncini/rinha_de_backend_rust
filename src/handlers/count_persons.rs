use actix_web::{get, HttpResponse, Responder};

#[get("/contagem-pessoas")]
async fn count_persons() -> impl Responder {
    HttpResponse::Ok()
}
