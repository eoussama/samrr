use normpath::PathExt;
use std::path::{Path, PathBuf};
use super::super::enums::error_enum::Error;

pub fn normalize(path: String) -> Result<PathBuf, Error> {
    let path = Path::new(&path);
    let op = |_| -> Error {Error::Other(format!("could not normalize path {:?}", path))};
    let path_norm = path.normalize().map_err(op).unwrap();

    Ok(path_norm.into_path_buf())
}
