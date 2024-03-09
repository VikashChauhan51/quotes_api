#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema,Clone)]
pub struct Quote {
    pub id: String,
    pub message: String,
}
