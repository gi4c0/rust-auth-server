use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub db: DBConfig,
}

#[derive(Deserialize)]
pub struct DBConfig {
    host: String,
    port: String,
    username: String,
    password: String,
    db_name: String,
}

impl DBConfig {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.username, &self.password, &self.host, &self.port, &self.db_name
        )
    }
}

pub fn parse_config() -> AppConfig {
    Config::builder()
        .add_source(config::File::with_name("config/local.json"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}
