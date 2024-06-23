use normpath::PathExt;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn normalize(path: String) -> Result<PathBuf, Error> {
    let path = Path::new(&path);
    let path_norm = path.normalize()?;

    Ok(path_norm.into_path_buf())
}
