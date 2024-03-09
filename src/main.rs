mod app;
mod config;
mod db;
mod models;
use crate::app::routes;
use actix_web::{web, App, HttpServer};
use env_logger::Builder;
use log::{info, trace, LevelFilter};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    trace!("quotes starting...");
    info!("quotes starting...");

    // get config
    let settings = config::Settings::new().unwrap();

    // Set the log level based on the settings
    let log_level = match settings.log.level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    };

    // init logger
    Builder::new().filter(None, log_level).init();

    // in memory collection
    let shared_map: Arc<RwLock<HashMap<String, models::quote::Quote>>> =
        Arc::new(RwLock::new(HashMap::new()));

    info!(
        "Server started at {}:{} and ENV:{:?}",
        settings.server.url, settings.server.port, settings.env
    );

    // init server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_map.clone()))
            .configure(routes::configure)
    })
    .bind(format!("{}:{}", settings.server.url, settings.server.port))?
    .run()
    .await
}
