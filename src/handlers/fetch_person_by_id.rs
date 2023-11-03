use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

use crate::{models::StoredPerson, AppState};

#[actix_web::get("/pessoas/{id}")]
async fn fetch_person_by_id(state: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    let person_by_id_result = sqlx::query_as::<_, StoredPerson>(
        "SELECT p.id, p.nickname, p.name, p.birth, p.stack FROM persons p WHERE p.id = $1",
    )
    .bind(id.into_inner())
    .fetch_one(&state.database_pool)
    .await;

    match person_by_id_result {
        Ok(person) => HttpResponse::Ok().json(person),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
