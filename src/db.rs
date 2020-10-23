use actix::Addr;
use actix_redis::RedisActor;

#[derive(Clone)]
pub struct DbManager {
	pub pool: sqlx::PgPool,
	pub redis: Addr<RedisActor>,
}

impl DbManager {
	pub async fn new(config: super::cfg::DatabaseConfig, redis: Addr<RedisActor>) -> anyhow::Result<Self> {
		log::debug!("Starting up Postgres pool");
		let pool = sqlx::PgPool::new(
			format!(
				"postgres://{}:{}@{}:{}/{}",
				config.user, config.password, config.host, config.port, config.dbname
			)
			.as_str(),
		)
		.await?;
		let this = Self { pool, redis };
		super::url_shortener::init_url_shortener(&this).await?;
		Ok(this)
	}

	pub async fn count_links(&self) -> ::anyhow::Result<i64> {
		let res = sqlx::query!("SELECT count(*) FROM links;")
			.fetch_one(&self.pool)
			.await?;
		Ok(res.count.unwrap())
	}
}

#[inline]
pub fn wrap_redis_err(error: actix_redis::Error) -> anyhow::Error {
	anyhow::Error::msg(format!("{}", error))
}
