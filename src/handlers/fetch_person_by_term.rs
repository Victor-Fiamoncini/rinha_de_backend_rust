use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct FetchPersonByTermQueryParams {
    pub t: Option<String>,
}

#[get("/pessoas")]
async fn fetch_person_by_term(
    state: Data<AppState>,
    query: web::Query<FetchPersonByTermQueryParams>,
) -> impl Responder {
    match &query.t {
        Some(term) => {
            if term.is_empty() || term.len() == 0 {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::BadRequest();
        }
    }

    HttpResponse::Ok()
}
