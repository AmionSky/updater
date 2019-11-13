#![allow(dead_code)] // Cause its annoying

mod config;
mod launcher;
mod locker;
mod provider;
mod update;
mod version;

use config::{Config, Verifiable};
use locker::Locker;
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::path::PathBuf;

fn main() {
    // Init base stuff
    setup_logger();
    let working_dir = get_working_dir().expect("failed to get the working directory");
    info!("Working directory: {}", working_dir.display());

    // Load configs
    let cfg = load_config();
    let mut should_launch = true;
    let mut version = version::read_file(version::app_file(&working_dir));

    // Launch if needed
    if !cfg.update.before_launch && version.is_some() {
        // Launch the application specified in the config
        launcher::launch(&working_dir, version.as_ref().unwrap(), &cfg.application);
        should_launch = false;
    }

    // Enable lockfile
    let mut locker = Locker::default();
    if !locker.lock() {
        info!("Process already running!");
        std::process::exit(0);
    }

    if version.is_some() || cfg.update.should_install {
        // Update
        let ver = version.unwrap_or_else(|| Version::new(0, 0, 0));
        version = match update::application(&working_dir, &cfg, ver) {
            Ok(v) => {
                if version::write_file(version::app_file(&working_dir), &v).is_err() {
                    error!("Failed to update version file");
                };
                Some(v)
            }
            Err(e) => {
                error!("Application update failed with error: {}", e);
                should_launch = false;
                None
            }
        }
    } else {
        should_launch = false;
    }

    // Launch if needed
    if should_launch {
        launcher::launch(&working_dir, version.as_ref().unwrap(), &cfg.application);
    }

    // Update self
    // TODO
}

/// Loads the configuration from file. Exits the program on error.
fn load_config() -> Config {
    let cfg = Config::load().unwrap_or_else(|e| {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    info!("Configuration loaded");
    cfg.verify().unwrap_or_else(|e| {
        error!("Config verification failed: {}", e);
        std::process::exit(2);
    });
    cfg
}

fn get_working_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    Ok(dir)
}

fn setup_logger() {
    use simplelog::{LevelFilter, SimpleLogger, TermLogger, TerminalMode};
    if TermLogger::init(LevelFilter::max(), logger_config(), TerminalMode::Mixed).is_err() {
        SimpleLogger::init(LevelFilter::Warn, logger_config()).expect("Logger failed to init")
    }
}

fn logger_config() -> simplelog::Config {
    let mut config = simplelog::ConfigBuilder::new();
    //config.set_target_level(simplelog::LevelFilter::max());
    config.add_filter_allow_str("updater");
    config.build()
}

/*
fn self_rename() {
    println!("Hello, world!");

    println!("{}", std::env::current_dir().unwrap().display());
    println!("{}", std::env::current_exe().unwrap().display());

    let mut to = std::env::current_exe().unwrap();
    to.set_extension("tmp");
    if std::fs::remove_file(&to).is_ok() {
        println!("Removed old tmp file");
    }
    std::fs::rename(std::env::current_exe().unwrap(), &to).unwrap();

    println!("{}", std::env::current_exe().unwrap().display());
}
*/
