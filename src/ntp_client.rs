use ntp::request;
use chrono::{DateTime, Utc, TimeZone};
use std::net::ToSocketAddrs;

pub fn get_ntp_time(server: &str) -> Result<DateTime<Utc>, String> {
    let addrs = server.to_socket_addrs().map_err(|e| format!("Failed to resolve NTP server address: {}", e))?;

    for addr in addrs {
        match request(addr) {
            Ok(packet) => {
                let ntp_seconds = packet.transmit_time.sec as i64;
                let unix_seconds = ntp_seconds - 2208988800;

                return Ok(Utc.timestamp_opt(unix_seconds, 0).unwrap());
            }
            Err(e) => {
                log::warn!("Failed to get NTP time from {:?}: {}", addr, e);
                continue;
            }
        }
    }

    Err("Failed to get NTP time from all resolved addresses".to_string())
}
