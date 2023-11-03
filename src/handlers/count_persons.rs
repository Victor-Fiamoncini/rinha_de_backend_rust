use actix_web::{http::header::ContentType, web::Data, HttpResponse, Responder};

use crate::{models::PersonsCount, AppState};

#[actix_web::get("/contagem-pessoas")]
async fn count_persons(state: Data<AppState>) -> impl Responder {
    let persons_count = sqlx::query_as::<_, PersonsCount>("SELECT COUNT(*) AS count FROM persons")
        .fetch_one(&state.database_pool)
        .await
        .unwrap_or_else(|_| PersonsCount { count: 0 });

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(persons_count.count.to_string())
}
