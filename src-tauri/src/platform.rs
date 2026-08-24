use std::process::Command;

#[cfg(target_os = "windows")]
pub fn open_external(target: &str) -> Result<(), String> {
    Command::new("rundll32.exe")
        .arg("url.dll,FileProtocolHandler")
        .arg(target)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open the link: {error}"))
}

#[cfg(target_os = "macos")]
pub fn open_external(target: &str) -> Result<(), String> {
    Command::new("open")
        .arg(target)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open the link: {error}"))
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn open_external(target: &str) -> Result<(), String> {
    Command::new("xdg-open")
        .arg(target)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open the link: {error}"))
}
