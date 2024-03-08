use actix_web::{get, HttpResponse, Responder};
use log::info;
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Response {
    data: String,
}


#[utoipa::path(
    responses(
        (status = 200, description = "API is alive and well!", body = Response)
    )
)]
#[get("/api/v1/index")]
pub async fn index() -> impl Responder {
    info!("index called.");

    HttpResponse::Ok()
        .content_type("application/json")
        .json("{'data':'Hello, world!'}")
}
