use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use zip::ZipArchive;

pub fn extract<P: AsRef<Path>>(zip: File, target: P) -> Result<(), Box<dyn Error>> {
    let mut archive = ZipArchive::new(zip)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target.as_ref().join(file.sanitized_name());

        if file.name().ends_with('/') {
            // Create directory
            fs::create_dir_all(&outpath)?;
        } else {
            // Create parent directory if needed
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            // Decompress file
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
