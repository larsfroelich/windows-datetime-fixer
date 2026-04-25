use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// NTP server address, e.g., "pool.ntp.org:123"
    pub ntp_server: String,
    /// Interval between time checks in minutes.
    pub check_interval_minutes: u64,
    /// Drift in seconds that triggers a system resync.
    pub drift_threshold_seconds: i64,
    /// Log level (error, warn, info, debug, trace)
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ntp_server: "pool.ntp.org:123".to_string(),
            check_interval_minutes: 12,
            drift_threshold_seconds: 10,
            log_level: "info".to_string(),
        }
    }
}

/// Returns the path to the application's configuration directory in %APPDATA%.
pub fn get_config_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA").map(|appdata| PathBuf::from(appdata).join("WDTF"))
    }
    #[cfg(not(windows))]
    {
        // Fallback for development/testing on non-windows platforms.
        directories::ProjectDirs::from("", "", "WDTF").map(|pd| pd.config_dir().to_path_buf())
    }
}

/// Loads the configuration from disk or creates a default one if it doesn't exist.
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

/// Saves the provided configuration to the standard location.
pub fn save_config(config: &Config) {
    let config_dir = get_config_dir().expect("Could not determine config directory");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    let content = toml::to_string_pretty(config).expect("Could not serialize config");
    fs::write(config_path, content).expect("Could not write config file");
}
