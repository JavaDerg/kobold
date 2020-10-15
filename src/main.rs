mod cfg;
mod db;

use crate::db::DbManager;
use actix_redis::RedisActor;
use actix_web::{middleware, App, HttpServer};
use env_logger::Env;

type STR = str;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

	log::warn!("{}", db::DbManager::GET_PEOPLE);

	let config = cfg::load()?;
	let pg_pool = DbManager::new(config.pg.create_pool(tokio_postgres::NoTls)?);
	let rd_pool = RedisActor::start(config.server.redis);

	let mut server = HttpServer::new(move || {
		App::new()
			.data(pg_pool.clone())
			.data(rd_pool.clone())
			.wrap(middleware::Logger::default())
	});
	for listener in &config.server.listeners {
		server = server.bind(listener)?;
	}
	server.run().await.map_err(|err| err.into())
}
