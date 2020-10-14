#[derive(Clone)]
pub struct DbManager {
	pool: deadpool_postgres::Pool,
}

impl DbManager {
	pub fn new(pool: deadpool_postgres::Pool) -> Self {
		Self { pool }
	}
}
