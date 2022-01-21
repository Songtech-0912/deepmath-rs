use anyhow::Result as AnyhowResult;
use flate2::read::GzDecoder;
use tar::Archive;
use std::fs;
use std::path::Path;

pub fn decompress(filepath: &Path, parent_dir: &Path, is_debug: bool) -> AnyhowResult<()> {
    let tar_gz = fs::File::open(&filepath)?;
    if is_debug {
        log::info!("Opened archive at {:?}", filepath);
    }
    let tar_decode = GzDecoder::new(tar_gz);
    if is_debug {
        log::info!("Created gz decoder");
    }
    let mut archive = Archive::new(tar_decode);
    archive.unpack(&parent_dir)?;
    if is_debug {
        log::info!("Decompressed {:?} to {:?}", filepath, parent_dir);
    }
    Ok(())
}