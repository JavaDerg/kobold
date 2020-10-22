mod dao;
mod dto;

use crate::db::DbManager;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/{key}")]
pub async fn resolve_url(db: web::Data<DbManager>, key: web::Path<String>) -> HttpResponse {
	if key.len() > 10 && key.len() < 6 {
		return HttpResponse::BadRequest().body("invalid key");
	}
	HttpResponse::Ok().body(key.as_ref())
}

#[post("/")]
pub async fn shorten_url(db: web::Data<DbManager>, body: web::Json<dto::ShortenRequest>) -> impl Responder {
	"2"
}
