use config::{Config, ConfigError, File};
use glob::glob;
use serde::Deserialize;
use std::collections::HashMap;

const CONFIG_FILE_PATH: &str = "./.suirc*";

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("config can be loaded");
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub aliases: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(
                glob(CONFIG_FILE_PATH)
                    .unwrap()
                    .map(|path| File::from(path.unwrap()))
                    .collect::<Vec<_>>(),
            )
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}
