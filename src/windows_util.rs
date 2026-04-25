#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use windows::Win32::Foundation::{HWND, HANDLE, CloseHandle};
#[cfg(windows)]
use windows::Win32::UI::Shell::ShellExecuteW;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONERROR, SW_SHOWNORMAL};
#[cfg(windows)]
use windows::Win32::System::Threading::{OpenProcessToken};
#[cfg(windows)]
use windows::core::HSTRING;
#[cfg(windows)]
use windows::Win32::Security::{TOKEN_QUERY, TokenElevation, GetTokenInformation};

use std::process::Command;
use std::env;

/// Checks if the current process has Administrator privileges.
#[cfg(windows)]
pub fn is_admin() -> bool {
    let mut token: HANDLE = HANDLE::default();
    unsafe {
        if OpenProcessToken(windows::Win32::System::Threading::GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation: u32 = 0;
            let mut size: u32 = 0;
            let _ = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<u32>() as u32,
                &mut size,
            );
            let _ = CloseHandle(token);
            elevation != 0
        } else {
            false
        }
    }
}

#[cfg(not(windows))]
pub fn is_admin() -> bool { false }

/// Re-launches the current application with Administrator privileges using the 'runas' verb.
#[cfg(windows)]
pub fn elevate_self() -> Result<(), String> {
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    let exe_path_u16: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();
    let runas_u16: Vec<u16> = OsStr::new("runas").encode_wide().chain(Some(0)).collect();

    unsafe {
        let result = ShellExecuteW(
            HWND(std::ptr::null_mut()),
            windows::core::PCWSTR(runas_u16.as_ptr()),
            windows::core::PCWSTR(exe_path_u16.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        );

        if result.0 as isize > 32 {
            Ok(())
        } else {
            Err(format!("ShellExecuteW failed with error code: {:?}", result.0))
        }
    }
}

#[cfg(not(windows))]
pub fn elevate_self() -> Result<(), String> { Err("Not implemented on this platform".to_string()) }

/// Triggers the Windows Time service to synchronize the system clock.
pub fn resync_time() -> Result<(), String> {
    // Ensure the Windows Time service (w32time) is running.
    let _ = Command::new("net").args(["start", "w32time"]).output();

    // Force a resynchronization.
    let output = Command::new("w32tm")
        .args(["/resync"])
        .output()
        .map_err(|e| format!("Failed to execute w32tm: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("w32tm failed: {}\n{}", stdout, stderr))
    }
}

/// Displays an error message box to the user.
pub fn show_error(message: &str) {
    #[cfg(windows)]
    {
        let title = HSTRING::from("WDTF Error");
        let msg = HSTRING::from(message);
        unsafe {
            MessageBoxW(HWND(std::ptr::null_mut()), &msg, &title, MB_OK | MB_ICONERROR);
        }
    }
    #[cfg(not(windows))]
    {
        eprintln!("Error: {}", message);
    }
}

/// Registers the application to run at startup by creating a shortcut in the Startup folder.
pub fn register_autostart() -> Result<(), String> {
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    #[cfg(windows)]
    {
        use std::path::PathBuf;
        let appdata = env::var("APPDATA").map_err(|e| e.to_string())?;
        let startup_folder = PathBuf::from(appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs\Startup");

        let shortcut_path = startup_folder.join("WDTF.url");

        if !shortcut_path.exists() {
            // A .url file is a simple way to create a shortcut without complex COM interfaces.
            // Using file:/// for local paths is supported by the Windows shell.
            let content = format!(
                "[InternetShortcut]\nURL=file:///{}\n",
                exe_path.to_string_lossy().replace('\\', "/")
            );

            std::fs::write(&shortcut_path, content).map_err(|e| format!("Failed to create startup shortcut: {}", e))?;
        }
    }
    #[cfg(not(windows))]
    {
        let _ = exe_path;
    }
    Ok(())
}
