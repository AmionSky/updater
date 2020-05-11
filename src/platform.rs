// Arch
#[cfg(target_arch = "x86")]
pub const ARCH: &str = "x86";
#[cfg(target_arch = "x86_64")]
pub const ARCH: &str = "x64";

// OS
#[cfg(target_os = "linux")]
pub const OS: &str = "linux";
#[cfg(target_os = "windows")]
pub const OS: &str = "win";
