// prevents a console window from appearing on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod logging;
mod ntp_client;
mod windows_util;

use chrono::Utc;
use std::thread;
use std::time::Duration;

fn main() {
    // Load config and init logging
    let config = config::load_config();
    logging::init_logging(&config.log_level);

    log::info!("WDTF started");

    // Register for autostart
    if let Err(e) = windows_util::register_autostart() {
        log::error!("Failed to register autostart: {}", e);
    } else {
        log::info!("Registered for autostart");
    }

    let mut error_halted = false;

    loop {
        if !error_halted {
            check_and_fix_time(&config, &mut error_halted);
        }

        // Sleep for the configured interval
        log::debug!("Sleeping for {} minutes", config.check_interval_minutes);
        thread::sleep(Duration::from_secs(config.check_interval_minutes * 60));
    }
}

fn check_and_fix_time(config: &config::Config, error_halted: &mut bool) {
    log::info!("Checking time sync...");

    match ntp_client::get_ntp_time(&config.ntp_server) {
        Ok(ntp_time) => {
            let local_time = Utc::now();
            let drift = (ntp_time - local_time).num_seconds().abs();

            log::info!("NTP time: {}, Local time: {}, Drift: {}s", ntp_time, local_time, drift);

            if drift > config.drift_threshold_seconds {
                log::warn!("Time drift detected ({}s), attempting to fix...", drift);

                if !windows_util::is_admin() {
                    log::info!("Not running as admin, attempting to elevate...");
                    if let Err(e) = windows_util::elevate_self() {
                        log::error!("Failed to elevate: {}", e);
                        windows_util::show_error(&format!("Failed to elevate for time synchronization: {}", e));
                        *error_halted = true;
                    } else {
                        log::info!("Elevation triggered, exiting this instance.");
                        std::process::exit(0);
                    }
                } else {
                    match windows_util::resync_time() {
                        Ok(_) => {
                            log::info!("Time synchronized successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to synchronize time: {}", e);
                            windows_util::show_error(&format!("Failed to synchronize time: {}", e));
                            *error_halted = true;
                        }
                    }
                }
            } else {
                log::info!("Time is within threshold ({}s)", config.drift_threshold_seconds);
            }
        }
        Err(e) => {
            log::error!("Failed to get NTP time: {}", e);
            // We don't necessarily halt on network error, just wait for next cycle
        }
    }
}
