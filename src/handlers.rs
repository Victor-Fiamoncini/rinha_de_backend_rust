use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use regex::Regex;
use serde::Deserialize;

use crate::models::NewPerson;

#[post("/pessoas")]
async fn store_person(new_person: web::Json<NewPerson>) -> impl Responder {
    if new_person.nickname.is_empty() || new_person.nickname.len() > 32 {
        return HttpResponse::BadRequest();
    }

    if new_person.name.is_empty() || new_person.name.len() > 100 {
        return HttpResponse::BadRequest();
    }

    let dateformat_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if new_person.birth.is_empty() || !dateformat_regex.is_match(&new_person.birth) {
        return HttpResponse::BadRequest();
    }

    match &new_person.stack {
        Some(stack) => {
            if stack.iter().any(|tech| tech.is_empty() || tech.len() > 32) {
                return HttpResponse::BadRequest();
            }
        }
        None => {}
    }

    HttpResponse::Created()
}

#[get("/pessoas/{id}")]
async fn fetch_person_by_id(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct FetchPersonByTermParams {
    pub t: String,
}

#[get("/pessoas")]
async fn fetch_person_by_term(query: web::Query<FetchPersonByTermParams>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/contagem-pessoas")]
async fn count_persons() -> impl Responder {
    HttpResponse::Ok()
}
