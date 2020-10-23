use crate::db::{wrap_redis_err, DbManager};
use actix_redis::Command;
use redis_async::resp::RespValue;
use redis_async::resp_array;

pub async fn resolve_url(db: &DbManager, key: &str) -> anyhow::Result<Option<String>> {
	if let some @ Some(_) = resolve_url_from_cache(db, key).await? {
		return Ok(some);
	}
	resolve_url_from_database(db, key).await;

	todo!()
}

async fn resolve_url_from_cache(db: &DbManager, key: &str) -> anyhow::Result<Option<String>> {
	let response = db
		.redis
		.send(Command(resp_array!["GET", key]))
		.await?
		.map_err(wrap_redis_err)?;
	Ok(match response {
		RespValue::Nil => None,
		RespValue::Array(array) => {
			log::error!("tried resolving key '{}' from redis, got array: {:?}", key, array);
			None
		}
		RespValue::BulkString(str) => Some(String::from_utf8(str)?),
		RespValue::Error(err) => {
			log::error!("tried resolving key '{}' from redis, got error: {:?}", key, err);
			None
		}
		RespValue::Integer(int) => {
			log::error!("tried resolving key '{}' from redis, got integer: {:?}", key, int);
			None
		}
		RespValue::SimpleString(str) => Some(str),
	})
}

async fn resolve_url_from_database(db: &DbManager, key: &str) -> anyhow::Result<super::dao::ResolveResponse> {
	Ok(sqlx::query_as!(
		super::dao::ResolveResponse,
		"SELECT * FROM links WHERE key = $1 LIMIT 1;",
		key
	)
	.fetch_one(&db.pool)
	.await?)
}

pub async fn init_url_shortener(db: &DbManager) -> anyhow::Result<()> {
	log::debug!("Initializing Database");
	/*
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
	*/
	sqlx::query!(
		r#"
			CREATE TABLE IF NOT EXISTS public.links
			(
				key TEXT PRIMARY KEY,
				created date NOT NULL,
				link TEXT NOT NULL,
				token UUID NOT NULL
			);
		"#
	)
	.execute(&db.pool)
	.await?;

	Ok(())
}
