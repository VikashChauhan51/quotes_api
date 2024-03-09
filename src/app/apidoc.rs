
use utoipa::OpenApi;
use crate::app::quote_handlers;
use crate::models;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};
// api doc and handlers and models configuration.
#[derive(OpenApi)]
    #[openapi(
        paths(
            quote_handlers::get_quotes, 
            quote_handlers::get_quote, 
            quote_handlers::add_quote, 
            quote_handlers::update_quote, 
            quote_handlers::delete_quote, 
        ),
        components(
            schemas(
                models::create_quote::CreateQuote,
                models::quote::Quote,
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