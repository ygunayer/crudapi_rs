use std::io::ErrorKind;

use account::service::AccountService;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use deadpool_postgres::{Config, Runtime};
use env_logger::Env;
use tokio_postgres::NoTls;

use crate::account::db::AccountRepository;
use crate::auth::service::AuthService;
use crate::settings::CoreApiConfig;

mod settings;
mod db;
mod common;
mod account;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let cfg = CoreApiConfig::load()
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

    let pool_config: Config = cfg.db.into();
    let db_pool = pool_config.create_pool(Some(Runtime::Tokio1), NoTls)
        .map_err(|e| std::io::Error::new(ErrorKind::ConnectionRefused, e))?;

    HttpServer::new(move || {
        let account_repository = AccountRepository::new(db_pool.clone());
        let auth_service = AuthService::new(account_repository.clone());
        let account_service = AccountService::new(account_repository);

        App::new()
            .app_data(Data::new(auth_service))
            .app_data(Data::new(account_service))
            .service(
            web::scope("api/v1/auth").configure(auth::endpoints::configure)
            )
    })
        .bind(cfg.server.url())?
        .run()
        .await
}
