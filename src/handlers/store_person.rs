use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use regex::Regex;
use uuid::Uuid;

use crate::{models::NewPerson, AppState};

#[post("/pessoas")]
async fn store_person(state: Data<AppState>, new_person: web::Json<NewPerson>) -> impl Responder {
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

    let mut stringfied_techs = String::new();

    match &new_person.stack {
        Some(stack) => {
            if stack.iter().any(|tech| tech.is_empty() || tech.len() > 32) {
                return HttpResponse::BadRequest();
            }

            stringfied_techs = stack.join(", ");
        }
        None => {}
    }

    let insert_result = sqlx::query(
        "INSERT INTO persons (id, nickname, name, birth, stack) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(&new_person.nickname)
    .bind(&new_person.name)
    .bind(&new_person.birth)
    .bind(stringfied_techs)
    .execute(&state.database_pool)
    .await;

    match insert_result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
