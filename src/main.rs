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
use std::error::Error;
use std::path::PathBuf;

fn main() {
    // Init base stuff
    simple_logger::init().expect("logger failed to initialize");
    let working_dir = working_dir().expect("failed to get the working directory");
    info!("Working directory: {}", working_dir.display());

    // Load config
    let cfg = load_config();

    // Launch the application specified in the config
    launcher::launch(&working_dir, &cfg.application);

    // Enable lockfile
    let mut locker = Locker::default();
    if !locker.lock() {
        info!("Process already running!");
        std::process::exit(0);
    }

    println!("{:#?}", cfg);
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

fn working_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    Ok(dir)
}
