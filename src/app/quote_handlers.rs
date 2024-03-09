use crate::models::create_quote::CreateQuote;
use crate::models::quote::Quote;
use crate::models::response::Response;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

type SharedMap = Arc<RwLock<HashMap<String, Quote>>>;
#[utoipa::path(
    responses(
        (status = 200, description = "Returns a list of quotes", body = Vec<Quote>)
    )
)]
#[get("/api/v1/quotes")]
pub async fn get_quotes(data: web::Data<SharedMap>) -> impl Responder {
    info!("get quotes called.");

    // Convert the HashMap to a Vec
    let read_guard = data.read().unwrap();
    let quotes_data: Vec<Quote> = read_guard.values().cloned().collect();
    let response: Response<Vec<Quote>> = Response { data: quotes_data };
    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "Returns a quote by id", body = Quote),
        (status = 404, description = "Returns a quote by id not found")
    )
)]
#[get("/api/v1/{id}/quote")]
pub async fn get_quote(data: web::Data<SharedMap>, path: web::Path<String>) -> impl Responder {
    info!("get quote called.");
    let id = path.into_inner();

    let read_guard = data.read().unwrap();
    match read_guard.get_key_value(&id) {
        Some(quote) => HttpResponse::Ok().json(quote.1),
        None => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
    responses(
        (status = 201, description = "Creates a new quote", body = CreateQuote)
    )
)]
#[post("/api/v1/quote")]
pub async fn add_quote(
    data: web::Data<SharedMap>,
    quote_model: web::Json<CreateQuote>,
) -> impl Responder {
    info!("add quote called.");
    print!("{:?}", quote_model.message);
    let mut write_guard = data.write().unwrap();
    let quote_uuid = Uuid::new_v4();
    write_guard.insert(
        quote_uuid.to_string(),
        Quote {
            id: quote_uuid.to_string(),
            message: quote_model.message.clone(),
        },
    );
    HttpResponse::Created().finish()
}

#[utoipa::path(
    responses(
        (status = 200, description = "Updates a quote by id", body = Quote),
        (status = 404, description = "Returns a quote by id not found")
    )
)]
#[put("/api/v1/{id}/quote")]
pub async fn update_quote(
    data: web::Data<SharedMap>,
    path: web::Path<String>,
    quote_model: web::Json<CreateQuote>,
) -> impl Responder {
    info!("update quote called.");
    let id = path.into_inner();
    print!("{}", id);
    print!("{:?}", quote_model.message);

    let mut write_guard = data.write().unwrap();
    match write_guard.get_mut(&id) {
        Some(quote) => {
            quote.message = quote_model.message.clone();
            HttpResponse::Ok().json(quote)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
    responses(
        (status = 204, description = "Deletes a quote by id")
    )
)]
#[delete("/api/v1/{id}/quote")]
pub async fn delete_quote(data: web::Data<SharedMap>, path: web::Path<String>) -> impl Responder {
    info!("delete quote called.");
    let id = path.into_inner();
    print!("{}", id);
    let mut write_guard = data.write().unwrap();
    match write_guard.remove(&id) {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().finish(),
    }
}
