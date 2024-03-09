#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema,Clone)]
pub struct CreateQuote {
    pub message: String,
}
