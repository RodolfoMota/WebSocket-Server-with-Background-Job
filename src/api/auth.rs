use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        message: "Hello, world!".to_string(),
    })
}
