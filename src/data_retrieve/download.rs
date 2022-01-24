use anyhow::Result as AnyhowResult;
use bytes::Bytes;
use reqwest::blocking::{Client, Response};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// Link to Facebook AI pre-built data archive
pub const TAR_URL: &str = "https://dl.fbaipublicfiles.com/SymbolicMathematics/data/prim_fwd.tar.gz";

pub fn init_download_dir(is_debug: bool) -> AnyhowResult<PathBuf> {
    let folder = env::temp_dir().join("deepmath_data");
    if is_debug {
        log::info!("Creating download folder {:?}", folder);
    };
    fs::create_dir_all(&folder)?;
    Ok(folder)
}

pub fn get_filename(is_debug: bool) -> AnyhowResult<PathBuf> {
    let request = make_request()?;
    let filename = request
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap();
    if is_debug {
        log::info!("Found filename of data: {:?}", filename)
    };
    Ok(PathBuf::from(filename))
}

pub fn get_filesize(is_debug: bool) -> AnyhowResult<u64> {
    let request = make_request()?;
    let filesize = request.content_length().unwrap();
    if is_debug {
        log::info!("Found file size of data: {:?}", filesize);
    };
    Ok(filesize)
}

pub fn make_request() -> AnyhowResult<Response> {
    // Must set timeout to none because downloading
    // the 1.7 GB data file is going to exceed standard timeout
    let request_client = Client::builder().timeout(None).build()?;
    let request = request_client.get(TAR_URL).send()?;
    Ok(request)
}

pub fn get_data(is_debug: bool) -> AnyhowResult<Bytes> {
    let request = make_request()?;
    let body = request.bytes()?;
    if is_debug {
        log::info!("Getting data from url");
    }
    Ok(body)
}

// Maybe future todo: implement progress bar for download (non-essential)
pub fn contents_to_file(bytes: Bytes, filepath: &Path, is_debug: bool) -> AnyhowResult<()> {
    let mut file = fs::File::create(&filepath)?;
    let mut content = std::io::Cursor::new(bytes);
    std::io::copy(&mut content, &mut file)?;
    if is_debug {
        log::info!("Copied request contents to file {:?}", filepath);
    }
    Ok(())
}
