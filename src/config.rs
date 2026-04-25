use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ntp_server: String,
    pub check_interval_minutes: u64,
    pub drift_threshold_seconds: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ntp_server: "pool.ntp.org:123".to_string(),
            check_interval_minutes: 12,
            drift_threshold_seconds: 10,
        }
    }
}

pub fn get_config_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA").map(|appdata| PathBuf::from(appdata).join("WDTF"))
    }
    #[cfg(not(windows))]
    {
        // Fallback for development on non-windows
        directories::ProjectDirs::from("", "", "WDTF").map(|pd| pd.config_dir().to_path_buf())
    }
}

pub fn load_config() -> Config {
    let config_dir = get_config_dir().expect("Could not determine config directory");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    if config_path.exists() {
        let content = fs::read_to_string(&config_path).expect("Could not read config file");
        toml::from_str(&content).unwrap_or_else(|_| {
            let default_config = Config::default();
            save_config(&default_config);
            default_config
        })
    } else {
        let default_config = Config::default();
        save_config(&default_config);
        default_config
    }
}

pub fn save_config(config: &Config) {
    let config_dir = get_config_dir().expect("Could not determine config directory");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    let content = toml::to_string_pretty(config).expect("Could not serialize config");
    fs::write(config_path, content).expect("Could not write config file");
}
