#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CreateQuote {
    pub message: String,
}
