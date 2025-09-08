use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    #[serde(alias = "type")]
    tp: String,
    host: String,
    database: String,
    port: u16,
    username: String,
    password: String,
}
impl Database {
    pub fn get_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.tp, self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
    pub server: Server,
    pub database: Database,
}
impl Setting {
    pub fn new() -> Result<Self, config::ConfigError> {
        let setting = Config::builder()
            .add_source(File::with_name("/config/dev"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        setting.try_deserialize()
    }
}

#[cfg(test)]
mod config_test {
    use anyhow::{self, Ok};

    use crate::config::Setting;
    #[test]
    fn init_config_test() -> anyhow::Result<()> {
        let setting = Setting::new()?;
        dbg!(setting);
        assert_eq!(1, 2);
        Ok(())
    }
}
