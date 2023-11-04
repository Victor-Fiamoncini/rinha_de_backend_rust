use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use regex::Regex;
use uuid::Uuid;

use crate::{dto::NewPersonDTO, AppState};

#[actix_web::post("/pessoas")]
async fn store_person(state: Data<AppState>, new_person_dto: Json<NewPersonDTO>) -> impl Responder {
    let stringfied_nickname: String;

    match &new_person_dto.nickname {
        Some(ref nickname) => {
            if let Some(nickname_str) = nickname.as_str() {
                if nickname_str.is_empty() || nickname_str.len() > 32 {
                    return HttpResponse::UnprocessableEntity().finish();
                }

                stringfied_nickname = nickname_str.to_string();
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity().finish();
        }
    }

    let stringfied_name: String;

    match &new_person_dto.name {
        Some(ref name) => {
            if let Some(name_str) = name.as_str() {
                if name_str.is_empty() || name_str.len() > 100 {
                    return HttpResponse::UnprocessableEntity().finish();
                }

                stringfied_name = name_str.to_string();
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity().finish();
        }
    }

    let stringfied_birth: String;

    match &new_person_dto.birth {
        Some(ref birth) => {
            if let Some(birth_str) = birth.as_str() {
                let dateformat_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

                if birth_str.is_empty() || !dateformat_regex.is_match(birth_str) {
                    return HttpResponse::UnprocessableEntity().finish();
                }

                stringfied_birth = birth_str.to_string();
            } else {
                return HttpResponse::BadRequest().finish();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity().finish();
        }
    }

    let mut vectorized_stack = Vec::new();

    match &new_person_dto.stack {
        Some(ref stack) => {
            if let Some(stack_vec) = stack.as_array() {
                for tech in stack_vec.iter() {
                    if let Some(tech_str) = tech.as_str() {
                        if tech_str.is_empty() || tech_str.len() > 32 {
                            return HttpResponse::UnprocessableEntity().finish();
                        }

                        vectorized_stack.push(tech_str);
                    } else {
                        return HttpResponse::BadRequest().finish();
                    }
                }
            }
        }
        None => {}
    }

    let person_id = Uuid::new_v4();

    let insert_result = state
        .database
        .insert_person(
            person_id,
            stringfied_nickname,
            stringfied_name,
            stringfied_birth,
            vectorized_stack,
        )
        .await;

    match insert_result {
        Ok(_) => HttpResponse::Created()
            .append_header(("Location", format!("/pessoas/{person_id}")))
            .finish(),
        Err(_) => HttpResponse::UnprocessableEntity().finish(),
    }
}
