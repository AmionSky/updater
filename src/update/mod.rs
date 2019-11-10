use log::error;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn self_exe() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

/// Replace file by renaming it to a temp name
fn replace_temp<P: AsRef<Path>>(replacement: P, target: P, temp: P) -> Result<(), Box<dyn Error>> {
    // First make sure the replacement exist before doing any work
    if fs::metadata(&replacement).is_err() {
        return Err("Replacement file does not exist!".into());
    }

    // Rename files
    fs::rename(&target, &temp)?;
    if let Err(e) = fs::rename(&replacement, &target) {
        // In case of error, undo the previous rename
        if fs::rename(&temp, &target).is_err() {
            error!("replace_temp failed to recover from error!");
        }

        return Err(e.into());
    }

    Ok(())
}
