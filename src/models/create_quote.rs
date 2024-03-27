use validator_derive::Validate;

#[derive(Debug, Validate, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct CreateQuote {
    #[validate(length(min = 10))]
    pub message: String,
}
