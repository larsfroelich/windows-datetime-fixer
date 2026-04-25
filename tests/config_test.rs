#[path = "../src/config.rs"]
mod config;

#[test]
fn test_default_config() {
    let cfg = config::Config::default();
    assert_eq!(cfg.ntp_server, "pool.ntp.org:123");
    assert_eq!(cfg.check_interval_minutes, 12);
    assert_eq!(cfg.drift_threshold_seconds, 10);
}
