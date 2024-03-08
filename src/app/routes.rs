use crate::app::apidoc;
use crate::app::handlers;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// config endpoints
pub fn configure(cfg: &mut web::ServiceConfig) {
    let openapi = apidoc::ApiDoc::openapi();

    cfg.service(handlers::index);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    );
}
