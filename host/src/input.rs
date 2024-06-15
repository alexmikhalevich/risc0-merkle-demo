use anyhow::Result;
use risc0_core::{Inputs, Page};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub directory: String,
    pub file_prefix: String,
    pub root_hash_file: String,
    pub num_pages: usize,
    pub page_size: usize,
}

fn read_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn load_config() -> Result<Config> {
    let config_filename =
        env::var("RISC0_DEMO_CONFIG").unwrap_or_else(|_| "config.yaml".to_string());
    let content = read_file_content(&config_filename)?;
    Ok(serde_yaml::from_slice(&content)?)
}

fn read_input_data(config: &Config) -> Result<Inputs> {
    let mut pages = Vec::new();
    for i in 1..config.num_pages + 1 {
        let filename = format!("{}{}", config.file_prefix, i);
        let path = Path::new(&config.directory).join(filename);
        let data = read_file_content(path)?;
        pages.push(Page { data });
    }
    let root_hash_path = Path::new(&config.directory).join(&config.root_hash_file);
    let merkle_root = read_file_content(root_hash_path)?;
    Ok(Inputs { pages, merkle_root })
}

pub fn read_input() -> Result<Inputs> {
    let config = load_config()?;
    read_input_data(&config)
}
