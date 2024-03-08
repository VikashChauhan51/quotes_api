
use utoipa::OpenApi;
use crate::app::handlers;
use crate::models;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};
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
        modifiers(&SecurityModifier)
    )]
    pub struct ApiDoc;

    //set auth
    pub struct SecurityModifier;
    impl Modify for SecurityModifier {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "basic_auth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
            );
        }
    }