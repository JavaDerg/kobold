#[derive(serde::Deserialize)]
pub struct ShortenRequest {}

#[derive(serde::Serialize)]
pub struct ShortenResponse {}

#[derive(serde::Serialize)]
pub struct ResolveResponse {
	pub key: String,
	pub link: String,
	pub created: chrono::NaiveDate,
}
