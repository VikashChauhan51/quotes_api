use actix_web::{get, HttpResponse, Responder};
use log::info;

#[get("/api/v1/index")]
pub async fn index() -> impl Responder {
    info!("index called.");

    HttpResponse::Ok()
        .content_type("application/json")
        .json("{'data':'Hello, world!'}")
}
