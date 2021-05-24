#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod launcher;
mod platform;
mod update;
mod version;

use config::{Config, Verifiable};
use log::{error, info, warn};
use semver::Version;
use std::error::Error;
use std::path::{Path, PathBuf};
use updater::Locker;

fn main() {
    #[cfg(target_os = "windows")]
    attach_console();

    setup_logger();
    let cfg = load_config();
    start(&cfg);
}

fn start(cfg: &Config) {
    let working_dir = get_working_dir().expect("failed to get the working directory");
    info!("Working directory: {}", working_dir.display());

    let mut version = version::read_file(version::app_file(&working_dir));
    if version.is_some() {
        // Check if the currently installed version's executable exist
        if !launcher::check(&working_dir, version.as_ref().unwrap(), &cfg.application) {
            warn!("Currently installed version not found!");
            version = None;
        } else {
            info!("Current version: {}", version.as_ref().unwrap());
        }
    }

    // Launch application if needed
    let mut should_launch = if !cfg.update.before_launch && version.is_some() {
        // Launch the application specified in the config
        launcher::launch(&working_dir, version.as_ref().unwrap(), &cfg.application);
        false
    } else {
        true
    };

    // Create lockfile
    let mut locker = Locker::default();
    // Exit if the updater is already running
    if !locker.lock() {
        info!("Process already running!");
        std::process::exit(0);
    }

    // try delete older versions
    info!("Cleaning-up older versions");
    if version.is_some() && clean_old_versions(&working_dir, version.as_ref().unwrap()).is_err() {
        error!("Failed to clean old version!");
    }

    // Update/Install application
    upd_app(&working_dir, &cfg, &mut should_launch, &mut version);

    // Launch application if needed
    if should_launch {
        launcher::launch(&working_dir, version.as_ref().unwrap(), &cfg.application);
    }

    // Update self
    if cfg.update.update_self {
        if let Err(err) = update::self_exe() {
            error!("Failed to update self: {}", err);
        }
    }
}

fn upd_app(wd: &Path, cfg: &Config, should_launch: &mut bool, version: &mut Option<Version>) {
    if version.is_some() || cfg.update.should_install {
        let ver = version.clone().unwrap_or_else(|| Version::new(0, 0, 0));
        *version = match update::application(&wd, &cfg, ver) {
            Ok(v) => {
                if version::write_file(version::app_file(&wd), &v).is_err() {
                    error!("Failed to update version file");
                };
                Some(v)
            }
            Err(e) => {
                error!("Application update failed with error: {}", e);
                *should_launch = false;
                None
            }
        }
    } else {
        *should_launch = false;
    }
}

/// Loads the configuration from file. Exits the program on error.
fn load_config() -> Config {
    let cfg = Config::load().unwrap_or_else(|e| {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    cfg.verify().unwrap_or_else(|e| {
        error!("Config verification failed: {}", e);
        std::process::exit(2);
    });
    info!("Configuration loaded and verified");
    cfg
}

fn get_working_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    Ok(dir)
}

fn clean_old_versions(wd: &Path, version: &Version) -> Result<(), Box<dyn Error>> {
    //let version_name = std::ffi::OsString::from(version.to_string());
    let dirs = std::fs::read_dir(wd)?;

    // Iterate over directories
    for dir in dirs.flatten() {
        // Get the directory name
        if let Some(dir_name) = dir.file_name().to_str() {
            // Try to convert the name to semver
            if let Ok(dir_version) = Version::parse(dir_name) {
                // Compare the dir_version to the current version
                if dir_version < *version {
                    // If its older, delete the directory
                    let dir_path = dir.path();
                    if std::fs::remove_dir_all(dir_path).is_err() {
                        error!("Failed to delete old version () directory!");
                    }
                }
            }
        }
    }

    Ok(())
}

fn setup_logger() {
    use simplelog::{ColorChoice, LevelFilter, SimpleLogger, TermLogger, TerminalMode};
    if TermLogger::init(
        LevelFilter::max(),
        logger_config(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .is_err()
    {
        SimpleLogger::init(LevelFilter::Warn, logger_config()).expect("Logger failed to init")
    }
}

fn logger_config() -> simplelog::Config {
    let mut config = simplelog::ConfigBuilder::new();
    //config.set_target_level(simplelog::LevelFilter::max());
    config.add_filter_allow_str("updater");
    config.build()
}

#[cfg(target_os = "windows")]
fn attach_console() {
    use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_working_dir() {
        let dir = get_working_dir().expect("get_working_dir() failed!");
        assert!(dir.is_dir());
    }
}
