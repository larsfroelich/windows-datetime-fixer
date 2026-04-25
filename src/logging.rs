use simplelog::*;
use std::fs::OpenOptions;
use crate::config::get_config_dir;

use std::str::FromStr;

pub fn init_logging(log_level: &str) {
    let config_dir = get_config_dir().expect("Could not determine config directory");
    let log_path = config_dir.join("wdtf.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Could not open log file");

    let level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Info);

    CombinedLogger::init(
        vec![
            TermLogger::new(level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(level, Config::default(), log_file),
        ]
    ).unwrap();
}
