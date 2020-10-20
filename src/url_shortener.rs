use actix_web::{get, post, Responder};

#[get("/{key}")]
pub async fn resolve_url() -> impl Responder {
	"oke!"
}

#[post("/")]
pub async fn shorten_url() -> impl Responder {
	"x3"
}
