#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod linux_mac;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use self::linux_mac::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;