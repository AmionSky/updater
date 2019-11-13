#[cfg(target_arch = "x86")]
pub const ARCH: &str = "x86";
#[cfg(target_arch = "x86_64")]
pub const ARCH: &str = "x86_64";

#[cfg_attr(linux, path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(macos, path = "macos.rs")]
mod os;
pub use os::*;
