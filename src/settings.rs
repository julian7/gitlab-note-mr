use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Clone, Deserialize)]
pub struct Target {
    pub user: String,
    pub pass: String,
    pub token: String,
}

#[derive(Clone, Deserialize)]
 pub struct Settings {
     pub bind: String,
     pub log: Log,
     pub gitlab_url: String,
     pub targets: Vec<Target>,
}

impl Settings {
    pub fn read(configfile: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("bind", "0.0.0.0:8000")?
            .set_default("gitlab_url", "https://gitlab.com")?
            .set_default("log.level", "info")?
            .add_source(File::with_name(&configfile[..]).required(false))
            .build()?;

        config.try_deserialize()
    }

    pub fn loglevel(self: &Self) -> usize {
        match self.log.level.as_str() {
        "off" => 0,
        "error" => 1,
        "warn" => 2,
        "info" => 3,
        "debug" => 4,
        "trace" => 5,
        _ => 3,
        }
    }
}
