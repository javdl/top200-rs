use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub eu_tickers: Vec<String>,
    pub us_tickers: Vec<String>,
    pub tickers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        // Try to read from config.toml first
        if let Ok(config) = load_config() {
            return config;
        }
        
        // Fallback to hardcoded defaults
        Self {
            eu_tickers: vec![
                "ASML".to_string(), "LVMH.PA".to_string(), "NOVO-B.CO".to_string(),
            ],
            us_tickers: vec![
                "NKE".to_string(), "TJX".to_string(), "VFC".to_string(),
            ],
            tickers: vec![
                "MC.PA".to_string(),     // LVMH (Paris)
                "NKE".to_string(),       // Nike (NYSE)
                "ITX.MC".to_string(),    // Inditex (Madrid)
            ],
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("config.toml");
    path
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_path = get_config_path();
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    let config_path = get_config_path();
    let config_str = toml::to_string_pretty(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}
