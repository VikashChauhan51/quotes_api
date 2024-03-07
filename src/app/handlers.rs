use actix_web::Responder;
use log::info;
pub async fn index() -> impl Responder {
    info!("index called.");
    "Hello, world!"
}