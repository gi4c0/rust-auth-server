use config::Config;
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

#[derive(Deserialize)]
pub struct Configuration {
    pub db: DBConfig,
    pub app: AppConfig,
}

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DBConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

impl DBConfig {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.db_name)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .password(&self.password)
            .username(&self.username)
            .host(&self.host)
            .port(self.port)
    }
}

pub fn parse_config() -> Configuration {
    Config::builder()
        .add_source(config::File::with_name("config/local.json"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}
