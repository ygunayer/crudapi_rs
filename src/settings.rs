use config::{Config, ConfigError, Environment, File};
use deadpool::managed::{PoolConfig, Timeouts};
use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DbConfig {
    hostname: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    max_pool_size: usize,
}

impl Into<deadpool_postgres::Config> for DbConfig {
    fn into(self) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.host = Some(self.hostname);
        cfg.port = Some(self.port);
        cfg.dbname = Some(self.database);
        cfg.user = Some(self.username);
        cfg.password = Some(self.password);
        cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
        cfg.pool = Some(PoolConfig { max_size: self.max_pool_size, timeouts: Timeouts::default() });

        cfg.into()
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfig {
    pub hostname: String,
    pub port: u16,
}

impl ServerConfig {
        pub fn url(&self) -> String {
                format!("{}:{}", self.hostname, self.port)
        }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CoreApiConfig {
    pub db: DbConfig,
    pub server: ServerConfig,
}

impl CoreApiConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or("development".into());

        let mut cfg = Config::new();

        // defaults
        cfg.merge(File::with_name("config/default"))?;

        // run mode specific
        cfg.merge(File::with_name(&format!("config/{}", run_mode)).required(false))?;

        // environment variables
        cfg.merge(Environment::with_prefix("CRUDAPI"))?;

        cfg.try_into()
    }
}
