mod app;
use log::{info, trace,LevelFilter};
use env_logger::Builder;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // init logger
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    trace!("quotes starting...");
    info!("quotes starting...");

    // init server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await   
}
