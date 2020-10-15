use tokio_postgres::types::Type;
macro_rules! define_sql_functions {
	(self, $($vis:vis async fn $name:ident($($arg:ident:$type:ty),*) -> $ret:ty => $query:literal;)*) => {
		$(
			::casey::shouty! {
				$vis const $name: &'static $crate::str = $query;
			}
			$vis async fn $name($($arg: $args),*) -> $ret { todo!() }
		)*
	};
}

#[derive(Clone)]
pub struct DbManager {
	pool: deadpool_postgres::Pool,
}

impl DbManager {
	pub fn new(pool: deadpool_postgres::Pool) -> Self {
		Self { pool }
	}

	pub async fn lol(&self) -> ::anyhow::Result<()> {
		let client = self.pool.get().await?;
		let stmt = client.prepare("").await?;
		client.query(&stmt, &[]);
		Ok(())
	}

	define_sql_functions! {
		self,
		pub async fn get_people() -> Vec<String> => "SELECT * FROM 'peoples'";
	}
}
