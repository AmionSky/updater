use super::Progress;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;



pub fn asset<P: AsRef<Path>>(
    name: &str,
    archive: File,
    target: P,
    progress: Arc<Progress>,
) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "ext-zip")]
    if name.ends_with(".zip") {
        return zip(archive, target, progress);
    }

    #[cfg(feature = "ext-targz")]
    if name.ends_with(".tar.gz") {
        return targz(archive, target, progress);
    }

    Err("Unknown archive format!".into())
}

#[cfg(feature = "ext-zip")]
pub fn zip<P: AsRef<Path>>(
    zip: File,
    target: P,
    progress: Arc<Progress>,
) -> Result<(), Box<dyn Error>> {
    use zip::ZipArchive;

    let mut archive = ZipArchive::new(zip)?;
    let mut size = 0;

    for i in 0..archive.len() {
        size += archive.by_index(i)?.size();
    }

    progress.add_maximum(size);
    progress.set_indeterminate(false);

    for i in 0..archive.len() {
        if progress.cancelled() {
            return Err("Extract cancelled!".into());
        }

        let mut zipped_item = archive.by_index(i)?;
        let out_path = target.as_ref().join(zipped_item.sanitized_name());

        if zipped_item.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            // Create parent directory if needed
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Decompress file
            let mut out_file = File::create(&out_path)?;
            std::io::copy(&mut zipped_item, &mut out_file)?;
        }

        // Get and Set permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = zipped_item.unix_mode() {
                std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(mode))?;
            }
        }

        progress.add_current(zipped_item.size());
    }

    Ok(())
}

#[cfg(feature = "ext-targz")]
pub fn targz<P: AsRef<Path>>(
    targz: File,
    target: P,
    progress: Arc<Progress>,
) -> Result<(), Box<dyn Error>> {
    use flate2::read::GzDecoder;
    use tar::Archive as TarArchive;

    let tar = GzDecoder::new(targz);
    let mut archive = TarArchive::new(tar);

    for entry in archive.entries()? {
        if progress.cancelled() {
            return Err("Extract cancelled!".into());
        }

        entry?.unpack_in(&target)?;
    }

    Ok(())
}
