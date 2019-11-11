#![allow(dead_code)] // Cause its annoying

mod config;
mod locker;
mod provider;
mod update;
mod version;

use config::Verifiable;
use locker::Locker;
use log::info;
use std::error::Error;
use std::path::PathBuf;

fn main() {
    simple_logger::init().unwrap();
    let mut locker = Locker::default();

    if !locker.lock() {
        info!("Process already running!");
        std::process::exit(0);
    }

    println!("{}", std::env::current_dir().unwrap().display());
    println!("{}", working_dir().unwrap().display());

    let cfg = config::load().unwrap();
    cfg.verify().unwrap();
    //println!("{:#?}", cfg);
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
