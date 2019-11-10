#![allow(dead_code)] // Cause its annoying

mod locker;
mod update;
mod version;

use locker::Locker;
use log::info;

fn main() {
    simple_logger::init().unwrap();
    let mut locker = Locker::default();

    if !locker.lock() {
        info!("Process already running!");
        std::process::exit(0);
    }
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
