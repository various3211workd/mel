#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod linux_markdown;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use self::linux_markdown::*;

#[cfg(target_os = "windows")]
pub mod markdown;
#[cfg(target_os = "windows")]
pub use self::markdown::*;