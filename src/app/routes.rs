use actix_web::web;
use crate::app::handlers;   

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::index);
}