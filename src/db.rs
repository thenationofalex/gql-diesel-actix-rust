extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use dotenv::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    init_pool(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
