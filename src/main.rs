mod cfg;
mod db;
mod url_shortener;

use crate::db::DbManager;
use actix_redis::RedisActor;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

	let config = cfg::load()?;
	let rd_pool = RedisActor::start(config.server.redis);
	let pg_pool = DbManager::new(config.pg, rd_pool).await?;

	if let Ok(count) = pg_pool.count_links().await {
		log::info!("Shortened Urls: {}", count);
	}

	let mut server = HttpServer::new(move || {
		App::new()
			.data(pg_pool.clone())
			.wrap(middleware::Logger::default())
			.service(
				web::scope("/api").service(
					web::scope("/v1").service(
						web::scope("url")
							.service(url_shortener::resolve_url)
							.service(url_shortener::shorten_url),
					),
				),
			)
	});
	for listener in &config.server.listeners {
		server = server.bind(listener)?;
	}
	server.run().await.map_err(|err| err.into())
}
