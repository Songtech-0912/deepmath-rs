use anyhow::Result as AnyhowResult;
use std::env;
use std::path::PathBuf;

pub fn model_location() -> AnyhowResult<PathBuf> {
    let folder = env::current_dir()?.join("deepmath_model/model.dat");
    Ok(folder)
}

pub fn model_present() -> AnyhowResult<bool> {
    Ok(model_location()?.exists())
}
