use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("postgres_pool")]
pub struct PostgresPool(sqlx::PgPool);

