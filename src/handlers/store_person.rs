use actix_web::{post, web, HttpResponse, Responder};
use regex::Regex;

use crate::models::NewPerson;

#[post("/pessoas")]
pub async fn store_person(new_person: web::Json<NewPerson>) -> impl Responder {
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
