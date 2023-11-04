use actix_web::{http::header::ContentType, web::Data, HttpResponse, Responder};

use crate::AppState;

#[actix_web::get("/contagem-pessoas")]
async fn count_persons(state: Data<AppState>) -> impl Responder {
    let count_result = state.database.count_persons().await;

    match count_result {
        Ok(ref person_count_model) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(person_count_model.count.to_string()),
        Err(_) => HttpResponse::UnprocessableEntity().finish(),
    }
}
