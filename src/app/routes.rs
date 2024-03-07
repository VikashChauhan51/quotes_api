use actix_web::web;
use crate::app::handlers;   

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index));
}