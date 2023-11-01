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
    let stringfied_nickname: String;

    match &new_person.nickname {
        Some(ref nickname) => {
            if let Some(nickname_str) = nickname.as_str() {
                if nickname_str.is_empty() || nickname_str.len() > 32 {
                    return HttpResponse::UnprocessableEntity();
                }

                stringfied_nickname = nickname_str.to_string();
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    let stringfied_name: String;

    match &new_person.name {
        Some(ref name) => {
            if let Some(name_str) = name.as_str() {
                if name_str.is_empty() || name_str.len() > 100 {
                    return HttpResponse::UnprocessableEntity();
                }

                stringfied_name = name_str.to_string();
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    let stringfied_birth: String;

    match &new_person.birth {
        Some(ref birth) => {
            if let Some(birth_str) = birth.as_str() {
                let dateformat_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

                if birth_str.is_empty() || !dateformat_regex.is_match(birth_str) {
                    return HttpResponse::UnprocessableEntity();
                }

                stringfied_birth = birth_str.to_string();
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    let mut vectorized_techs = Vec::new();

    match &new_person.stack {
        Some(ref stack) => {
            if let Some(stack_vec) = stack.as_array() {
                for tech in stack_vec.iter() {
                    if let Some(tech_str) = tech.as_str() {
                        if tech_str.is_empty() || tech_str.len() > 32 {
                            return HttpResponse::UnprocessableEntity();
                        }

                        vectorized_techs.push(tech_str);
                    } else {
                        return HttpResponse::BadRequest();
                    }
                }
            }
        }
        None => {}
    }

    let insert_result = sqlx::query(
        "INSERT INTO persons (id, nickname, name, birth, stack) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(stringfied_nickname)
    .bind(stringfied_name)
    .bind(stringfied_birth)
    .bind(vectorized_techs)
    .execute(&state.database_pool)
    .await;

    match insert_result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::UnprocessableEntity(),
    }
}
