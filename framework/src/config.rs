use common::AppError;
use config::{Config, Environment, File};
use salvo::oapi::schema;
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
    sechma: Option<String>,
    port: u16,
    username: String,
    password: String,
}
impl Database {
    pub fn get_url(&self) -> String {
        match &self.sechma {
            Some(schema) => format!(
                "{}://{}:{}@{}:{}/{}?options=-c search_path={}",
                self.tp, self.username, self.password, self.host, self.port, self.database, schema
            ),
            None => format!(
                "{}://{}:{}@{}:{}/{}",
                self.tp, self.username, self.password, self.host, self.port, self.database
            ),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWT {
    pub secret: String,
    pub issuer: String,
    pub acc_expiration_hour: u8,
    pub ref_expiration_hour: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
    pub server: Server,
    pub database: Database,
    pub jwt: JWT,
}
impl Setting {
    pub fn init() -> Result<Self, AppError> {
        let setting = Config::builder()
            .add_source(File::with_name("config/dev"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        let setting = setting.try_deserialize()?;
        Ok(setting)
    }
}

#[cfg(test)]
mod config_test {
    use anyhow::{self, Ok};

    use crate::config::Setting;
    #[test]
    fn init_config_test() -> anyhow::Result<()> {
        let setting = Setting::init()?;
        dbg!(setting);
        assert_eq!(1, 2);
        Ok(())
    }
}
