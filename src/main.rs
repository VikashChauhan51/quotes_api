mod app;
use crate::app::routes;
use actix_web::{App, HttpServer};
use env_logger::Builder;
use log::{info, trace, LevelFilter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    Builder::new().filter(None, LevelFilter::Info).init();

    trace!("quotes starting...");
    info!("quotes starting...");

    // init server
    HttpServer::new(|| App::new().configure(routes::configure))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
