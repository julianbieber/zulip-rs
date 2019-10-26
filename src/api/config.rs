use std::path::PathBuf;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZulipConfig {
    pub user: String,
    pub domain: String,
    pub password: String,
}

lazy_static! {
    pub static ref ZULIP_CONFIG_PATH: PathBuf = dirs::home_dir().unwrap_or(Path::new(".").to_path_buf()).join(Path::new(".zulip_rs/config.toml"));
}

impl ZulipConfig {
    pub fn from_file(path: PathBuf) -> Result<ZulipConfig, Error> {

        let toml_string = std::fs::read_to_string(path)?;
        let config = toml::from_str(toml_string.as_str()).map_err(|e| {
           Error::from(e)
        });
        config
    }
}