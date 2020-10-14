mod cfg;

use actix_redis::RedisActor;
use actix_web::{middleware, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

	let config = cfg::load()?;
	let pg_pool = config.pg.create_pool(tokio_postgres::NoTls)?;
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
