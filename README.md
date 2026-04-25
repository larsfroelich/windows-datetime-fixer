# Windows Date/Time Fixer (WDTF)

A minimal, headless Rust application for Windows 11 that ensures your system clock stays synchronized with high-precision NTP servers.

## Features

- **Minimal Footprint**: Uses a single thread and standard system sleep to ensure zero CPU usage between checks. The binary is optimized for size (~600KB).
- **Headless Operation**: Runs entirely in the background with no console window or system tray icon.
- **On-Demand Elevation**: Only requests Administrator privileges when necessary to synchronize the system clock.
- **Auto-Registration**: Automatically registers itself to run at startup by creating a shortcut in the Windows Startup folder.
- **Configurable**: Easily adjust the NTP server, check interval, and drift threshold via a TOML configuration file.
- **Robust Logging**: Keeps track of synchronization attempts and errors in a persistent log file.

## Requirements

- **Operating System**: Windows 11
- **Privileges**: Normal user for background operation; Administrator prompt will appear only when time correction is required.

## Installation
For now, File Kraken is still in development and there is no initial release yet.
To build and install directly from source, you can use

`cargo install --git https://github.com/larsfroelich/windows-datetime-fixer.git`

Then, run the application once. It will:
   - Create a configuration folder at `%APPDATA%\WDTF`.
   - Register itself for autostart.
   - Perform an initial time check.

## Configuration

The configuration file is located at `%APPDATA%\WDTF\config.toml`.

```toml
# The NTP server to query (including port)
ntp_server = "pool.ntp.org:123"

# How often to check the time (in minutes)
check_interval_minutes = 12

# Maximum allowed drift (in seconds) before triggering a fix
drift_threshold_seconds = 10

# Log level (error, warn, info, debug, trace)
log_level = "info"
```

## Logs

Activity and error logs can be found at `%APPDATA%\WDTF\wdtf.log`.

## How it Works

1. **Startup**: On launch, WDTF ensures it is registered in the Windows Startup folder.
2. **Detection**: Every 12 minutes (default), it queries the configured NTP server and calculates the difference (drift) between the network time and local system time.
3. **Correction**: If the drift exceeds 10 seconds (default):
   - WDTF checks if it has Administrator privileges.
   - If not, it re-launches itself with a UAC prompt (`runas`).
   - The elevated instance starts the `w32time` service and triggers `w32tm /resync`.
4. **Error Handling**: If synchronization fails (e.g., no internet or service error), it displays a message box and stops automatic checks to prevent repeated UAC prompts, until the next restart.
