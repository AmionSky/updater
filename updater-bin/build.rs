use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let appname = std::env::var("CARGO_PKG_VERSION_MAJOR").expect("Failed to get application name");
        embed_manifest(new_manifest(&appname)).expect("unable to embed manifest file");
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}