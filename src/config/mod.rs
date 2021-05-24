use tokio::fs::File;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::io::ErrorKind::NotFound;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use thiserror::Error;

const CONFIG_PATH: &str = "config.yml";
const DEFAULT_CONFIG: &str = include_str!("default.yml");

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub motd: String,
}

impl Config {
    pub async fn load() -> Result<Config> {
        match File::open(CONFIG_PATH).await {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).await?;
                Self::from_str(content.as_str())
            },
            Err(e) if e.kind() == NotFound => {
                create_default_file().await;
                Self::from_str(DEFAULT_CONFIG)
            },
            Err(e) => Err(e.into()),
        }
    }

    fn from_str(source: &str) -> Result<Config> {
        let yaml = &YamlLoader::load_from_str(source)?[0];
        Ok(Self {
            port: yaml["port"].as_i64().map(|port| port as u16)
                .ok_or(MissingField("port"))?,
            motd: yaml["motd"].as_str().map(|motd| String::from(motd))
                .ok_or(MissingField("motd"))?,
        })
    }
}

async fn create_default_file() -> Result<()> {
    let mut file = File::create(CONFIG_PATH).await?;
    file.write_all(DEFAULT_CONFIG.as_bytes()).await?;
    Ok(())
}

#[derive(Error, Debug)]
#[error("missing or invalid field: {0}")]
struct MissingField(&'static str);