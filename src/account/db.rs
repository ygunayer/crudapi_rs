use std::result::Result;
use std::time::SystemTime;

use async_trait::async_trait;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

use crate::db::Connection;
use crate::db::error::DbError;
use crate::db::repository::Repository;

use crudapi_derive::Repository;

type DateTime = SystemTime;

#[derive(PostgresMapper)]
#[pg_mapper(table = "account")]
pub struct AccountEntity {
    pub id: i64,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_master: bool,
    pub activation_token: Option<String>,
    pub activation_token_expires_at: Option<DateTime>
}

#[derive(Clone, Repository)]
#[repository("account", AccountEntity, i64)]
pub struct AccountRepository {
    conn: Connection
}

impl AccountRepository {
    pub fn new(conn: Connection) -> AccountRepository {
        AccountRepository { conn }
    }

    pub async fn find_by_email(&self, email: &String) -> Result<Option<AccountEntity>, DbError> {
        self.select_one("SELECT * FROM account WHERE email = $1 AND status <> -1", &[&email]).await
    }

    pub async fn exists_by_email(&self, email: &String) -> Result<bool, DbError> {
        self.exists("SELECT * FROM account WHERE email = $1 AND status <> -1", &[&email]).await
    }
}
