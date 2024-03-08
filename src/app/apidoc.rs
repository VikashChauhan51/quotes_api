
use utoipa::OpenApi;
use crate::app::handlers;
use crate::models;

// api doc and handlers and models configuration.
#[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::index, // handlers endpoints
        ),
        components(
            schemas(
                models::response::Response //schema models
            ),
        ),
    )]
    pub struct ApiDoc;