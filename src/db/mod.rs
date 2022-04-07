use deadpool_postgres::Pool;

pub mod repository;
pub mod error;

pub type Connection = Pool;
