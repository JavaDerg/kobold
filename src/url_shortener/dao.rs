pub struct ShortenRequest {}

pub struct ShortenResponse {}

pub struct ResolveResponse {
	pub key: String,
	pub link: String,
	pub created: chrono::NaiveDate,
	pub token: uuid::Uuid,
}

impl Into<super::dto::ResolveResponse> for ResolveResponse {
	fn into(self) -> super::dto::ResolveResponse {
		super::dto::ResolveResponse {
			key: self.key,
			link: self.link,
			created: self.created,
		}
	}
}
