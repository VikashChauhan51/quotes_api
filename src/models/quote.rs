#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Quote {
    pub id: String,
    pub message: String,
}
