use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{models::StoredPerson, AppState};

#[derive(Deserialize)]
struct FetchPersonByTermQueryParams {
    #[serde(rename(deserialize = "t"))]
    pub search: Option<String>,
}

#[get("/pessoas")]
async fn fetch_person_by_term(
    state: Data<AppState>,
    query: web::Query<FetchPersonByTermQueryParams>,
) -> impl Responder {
    let stringfied_query: String;

    match &query.search {
        Some(ref term) => {
            if term.is_empty() || term.len() == 0 {
                return HttpResponse::BadRequest().json(());
            }

            stringfied_query = term.to_string();
        }
        None => {
            return HttpResponse::BadRequest().json(());
        }
    }

    let persons_by_term = sqlx::query_as::<_, StoredPerson>(
        "SELECT p.id, p.nickname, p.name, p.birth, p.stack FROM persons p WHERE p.search_terms ILIKE $1 LIMIT 50",
    )
    .bind(format!("%{stringfied_query}%"))
    .fetch_all(&state.database_pool)
    .await;

    match persons_by_term {
        Ok(persons) => HttpResponse::Ok().json(persons),
        Err(_) => HttpResponse::NotFound().json(()),
    }
}
