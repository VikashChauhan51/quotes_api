use crate::app::apidoc;
use crate::app::quote_handlers;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// config endpoints
pub fn configure(cfg: &mut web::ServiceConfig) {
    let openapi = apidoc::ApiDoc::openapi();

    cfg.service(quote_handlers::get_quotes);
    cfg.service(quote_handlers::get_quote);
    cfg.service(quote_handlers::add_quote);
    cfg.service(quote_handlers::update_quote);
    cfg.service(quote_handlers::delete_quote);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    );
}
