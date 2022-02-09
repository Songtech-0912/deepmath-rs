use anyhow::Result as AnyhowResult;
use std::env;
use std::path::PathBuf;

pub fn data_location() -> AnyhowResult<PathBuf> {
    let folder = env::current_dir()?.join("deepmath_data");
    Ok(folder)
}

pub fn data_present() -> AnyhowResult<bool> {
    Ok(true)
}
