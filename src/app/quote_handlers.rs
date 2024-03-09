use std::any::Any;

use crate::models::create_quote::CreateQuote;
use crate::models::quote::Quote;
use crate::models::response::Response;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::info;

#[utoipa::path(
    responses(
        (status = 200, description = "Returns a list of quotes", body = Vec<Quote>)
    )
)]
#[get("/api/v1/quotes")]
pub async fn get_quotes() -> impl Responder {
    info!("get quotes called.");
    // You would typically fetch quotes from your database here
    let quotes_data: Vec<Quote> = vec![];
    let response: Response<Vec<Quote>> = Response { data: quotes_data };
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "Returns a quote by id", body = Quote)
    )
)]
#[get("/api/v1/{id}/quote")]
pub async fn get_quote(path: web::Path<String>) -> impl Responder {
    info!("get quote called.");
    let id = path.into_inner();
    // You would typically fetch the quote with the provided id from your database here
    let quote_data = Quote {
        id,
        message: String::from("Hello, world!"),
    };
    HttpResponse::Ok().json(quote_data)
}

#[utoipa::path(
    responses(
        (status = 201, description = "Creates a new quote", body = CreateQuote)
    )
)]
#[post("/api/v1/quote")]
pub async fn add_quote(quote_model: web::Json<CreateQuote>) -> impl Responder {
    info!("add quote called.");
    print!("{:?}", quote_model.message);
    HttpResponse::Created().finish()
}

#[utoipa::path(
    responses(
        (status = 200, description = "Updates a quote by id", body = Quote)
    )
)]
#[put("/api/v1/{id}/quote")]
pub async fn update_quote(
    path: web::Path<String>,
    quote_model: web::Json<CreateQuote>,
) -> impl Responder {
    info!("update quote called.");
    let id = path.into_inner();
    print!("{}", id);
    print!("{:?}", quote_model.message);
    HttpResponse::Ok().json(quote_model)
}

#[utoipa::path(
    responses(
        (status = 204, description = "Deletes a quote by id")
    )
)]
#[delete("/api/v1/{id}/quote")]
pub async fn delete_quote(path: web::Path<String>) -> impl Responder {
    info!("delete quote called.");
    let id = path.into_inner();
    print!("{}", id);
    HttpResponse::NoContent().finish()
}
