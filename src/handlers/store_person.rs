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
    match &new_person.nickname {
        Some(nickname) => {
            if let Some(nickname_str) = nickname.as_str() {
                if nickname_str.is_empty() || nickname_str.len() > 32 {
                    return HttpResponse::UnprocessableEntity();
                }
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    match &new_person.name {
        Some(name) => {
            if let Some(name_str) = name.as_str() {
                if name_str.is_empty() || name_str.len() > 100 {
                    return HttpResponse::UnprocessableEntity();
                }
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    match &new_person.birth {
        Some(birth) => {
            if let Some(birth_str) = birth.as_str() {
                let dateformat_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

                if birth_str.is_empty() || !dateformat_regex.is_match(birth_str) {
                    return HttpResponse::UnprocessableEntity();
                }
            } else {
                return HttpResponse::BadRequest();
            }
        }
        None => {
            return HttpResponse::UnprocessableEntity();
        }
    }

    let mut stringfied_techs = String::new();

    match &new_person.stack {
        Some(stack) => {
            if let Some(stack_vec) = stack.as_array() {
                for (index, tech) in stack_vec.iter().enumerate() {
                    if let Some(tech_str) = tech.as_str() {
                        if tech_str.is_empty() || tech_str.len() > 32 {
                            return HttpResponse::UnprocessableEntity();
                        }

                        stringfied_techs.push_str(&tech_str);

                        if index < stack_vec.len() - 1 {
                            stringfied_techs.push_str(", ");
                        }
                    } else {
                        return HttpResponse::BadRequest();
                    }
                }
            }
        }
        None => {}
    }

    println!("{}", stringfied_techs);

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
        Err(err) => {
            println!("{}", err);

            return HttpResponse::UnprocessableEntity();
        }
    }
}
