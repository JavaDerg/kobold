mod dao;
mod dto;
mod service;

pub use service::init_url_shortener;

use crate::db::DbManager;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/{key}")]
pub async fn resolve_url(db: web::Data<DbManager>, key: web::Path<String>) -> HttpResponse {
	if key.len() > 10 && key.len() < 6 {
		return HttpResponse::BadRequest().body("invalid key");
	}
	match service::resolve_url(db.get_ref(), key.as_ref()).await {
		Ok(Some(url)) => HttpResponse::Ok().json(""),
		Ok(None) => HttpResponse::NotFound().finish(),
		Err(err) => {
			log::warn!("{}", err);
			HttpResponse::InternalServerError().finish()
		}
	}
}

#[post("/")]
pub async fn shorten_url(db: web::Data<DbManager>, body: web::Json<dto::ShortenRequest>) -> HttpResponse {
	todo!()
}
