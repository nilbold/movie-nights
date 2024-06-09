use config::{Config, File};
use glob::glob;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Database {
    pub name: String,
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn read() -> anyhow::Result<AppConfig> {
        let config = Config::builder()
            .add_source(
                glob("./config/*")?
                    .map(|path| File::from(path.unwrap()))
                    .collect::<Vec<_>>(),
            )
            .build()?;

        Ok(config.try_deserialize::<AppConfig>()?)
    }
}

impl Database {
    /// Generates a postgres connection string from the config
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.name
        )
    }

    /// Generates a connection string while omitting the database name
    pub fn conncetion_string_without_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.pass, self.host, self.port
        )
    }
}
