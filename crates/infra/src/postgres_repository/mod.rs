use anyhow::{Ok, Result};
use async_trait::async_trait;
use domain::{entity::W3Transaction, repository::database_repository::DatabaseRepository};
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    query_as, FromRow, PgPool, Pool, Postgres,
};
use std::env;

pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub async fn new() -> Self {
        let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = Pool::connect(&pg_url).await.unwrap();
        Self { pool }
    }

    async fn raw_query<T>(
        &self,
        query: sqlx::query::QueryAs<'_, Postgres, T, PgArguments>,
    ) -> Result<Vec<T>>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin + 'static,
    {
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn raw_update(&self, query: Query<'_, Postgres, PgArguments>) -> Result<()> {
        let rs = query.execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl DatabaseRepository for PostgresRepository {
    //get transaction of user over an address
    async fn get_related_transaction(&self, address: &str) -> Result<Vec<W3Transaction>> {
        let query_str =
            format!("select w3_address, w3_height from w3_transaction where w3_address = $1");
        let query_builder = query_as::<_, W3Transaction>(&query_str).bind(address);
        let rs = self.raw_query(query_builder).await?;
        Ok(rs)
    }

    //update new transaction of user
    async fn insert_new_transaction(&self, height: &str, address: &str) -> Result<()> {
        let query_str =
            format!("insert into w3_transaction (w3_height, w3_address) values ($1, $2)");
        let query_buidler = sqlx::query(&query_str).bind(height).bind(address);
        let rs = self.raw_update(query_buidler).await?;
        Ok(())
    }
}
