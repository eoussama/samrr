use normpath::PathExt;
use std::path::{Path, PathBuf};

use super::error;

pub fn normalize(path: String) -> Result<PathBuf, error::Error> {
    let path = Path::new(&path);
    let op = |_| error::Error::Other(format!("could not normalize path {:?}", path));
    let path_norm = path.normalize().map_err(op).unwrap();

    Ok(path_norm.into_path_buf())
}
