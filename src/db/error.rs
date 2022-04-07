use deadpool::managed::PoolError;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct DbError {
    message: String
}

impl From<PoolError<tokio_postgres::Error>> for DbError {
    fn from(err: PoolError<Error>) -> Self {
        DbError {
            message: format!("Database connection failed due to a connection pool error: {:?}", err)
        }
    }
}

impl From<tokio_postgres::Error> for DbError {
    fn from(err: Error) -> Self {
        DbError {
            message: format!("Failed to communicate with the database server: {:?}", err)
        }
    }
}

impl From<tokio_pg_mapper::Error> for DbError {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        DbError {
            message: format!("Failed to map database entities: {:?}", err)
        }
    }
}
