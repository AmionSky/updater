mod application;
mod selfexe;

pub use application::application;
pub use selfexe::self_exe;

pub fn convert_asset_name(name: &str) -> String {
    use crate::platform::{ARCH, OS};
    let new_name = name.replace("<os>", OS);
    new_name.replace("<arch>", ARCH)
}
