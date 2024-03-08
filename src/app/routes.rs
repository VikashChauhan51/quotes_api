use crate::app::handlers;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// api doc and handlers and models configuration.
#[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::index,
        ),
        components(
            schemas(
                handlers::Response
            ),
        ),
    )]
    pub struct ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(handlers::index);
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    );
}
