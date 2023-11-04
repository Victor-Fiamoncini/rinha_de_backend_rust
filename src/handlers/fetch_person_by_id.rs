use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

use crate::AppState;

#[actix_web::get("/pessoas/{id}")]
async fn fetch_person_by_id(state: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    let select_result = state.database.select_person_by_id(id.into_inner()).await;

    match select_result {
        Ok(ref person) => HttpResponse::Ok().json(person),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
