#[derive(Clone)]
pub struct DbManager {
	pool: sqlx::PgPool,
}

impl DbManager {
	pub async fn new(config: super::cfg::DatabaseConfig) -> anyhow::Result<Self> {
		log::debug!("Starting up Postgres pool");
		let pool = sqlx::PgPool::new(
			format!(
				"postgres://{}:{}@{}:{}/{}",
				config.user, config.password, config.host, config.port, config.dbname
			)
			.as_str(),
		)
		.await?;
		init_url_shortener(&pool).await?;
		Ok(Self { pool })
	}

	pub async fn count_links(&self) -> ::anyhow::Result<i64> {
		let res = sqlx::query!("SELECT count(*) FROM links;")
			.fetch_one(&self.pool)
			.await?;
		Ok(res.count.unwrap())
	}
}

async fn init_url_shortener(pool: &sqlx::PgPool) -> anyhow::Result<()> {
	log::debug!("Initializing Database");
	sqlx::query!(
		r#"
DO $$
BEGIN
	IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'link_redirect') THEN
		CREATE TYPE link_redirect AS ENUM ('http', 'js', 'captcha');
	END IF;
END$$;
	"#
	)
	.execute(pool)
	.await?;
	sqlx::query!(
		r#"
CREATE TABLE IF NOT EXISTS public.links
(
    key TEXT PRIMARY KEY,
    link TEXT NOT NULL,
	token UUID NOT NULL,
	type link_redirect NOT NULL
);
	"#
	)
	.execute(pool)
	.await?;

	Ok(())
}
