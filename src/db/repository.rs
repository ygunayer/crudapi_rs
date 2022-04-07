use async_trait::async_trait;
use tokio_postgres::{types::ToSql, Row};
use crate::db::error::DbError;

#[async_trait]
pub trait Repository<T, ID = i64> {
    async fn select_row(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, DbError>;

    async fn select_one(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Option<T>, DbError>;

    async fn find_by_id(&self, id: ID) -> Result<Option<T>, DbError>;

    async fn exists(&self, query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<bool, DbError>;
}
