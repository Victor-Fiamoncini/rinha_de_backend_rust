use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct QueryParams {
    #[serde(rename(deserialize = "t"))]
    pub search: Option<String>,
}

#[actix_web::get("/pessoas")]
async fn fetch_person_by_term(state: Data<AppState>, query: Query<QueryParams>) -> impl Responder {
    let stringfied_query: String;

    match &query.search {
        Some(ref term) => {
            if term.is_empty() || term.len() == 0 {
                return HttpResponse::BadRequest().finish();
            }

            stringfied_query = term.to_string();
        }
        None => {
            return HttpResponse::BadRequest().finish();
        }
    }

    let select_result = state.database.select_person_by_term(stringfied_query).await;

    match select_result {
        Ok(ref persons) => HttpResponse::Ok().json(persons),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
