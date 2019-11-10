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

fn locker_tester(locker: &mut Locker) {
    println!("Locker state: {}", locker.is_locked());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Lock result: {}", locker.lock());
    println!("Locker state: {}", locker.is_locked());
    println!("UnLock result: {}", locker.unlock());
    println!("Locker state: {}", locker.is_locked());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("UnLock result: {}", locker.unlock());
    println!("Locker state: {}", locker.is_locked());
    println!("Lock result: {}", locker.lock());
    println!("Locker state: {}", locker.is_locked());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
