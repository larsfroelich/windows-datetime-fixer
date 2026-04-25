use simplelog::*;
use std::fs::OpenOptions;
use crate::config::get_config_dir;

pub fn init_logging() {
    let config_dir = get_config_dir().expect("Could not determine config directory");
    let log_path = config_dir.join("wdtf.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Could not open log file");

    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).unwrap();
}
