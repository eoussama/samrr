use std::io::{Error, ErrorKind};
use std::path::Path;

use libloading::Library;

fn set_sources(path: &Path) {
    let path_bin = path.join("bin");

    super::env_helper::set_path(path);
    super::env_helper::set_path(&path_bin);
}

pub fn load(path: String, name: &str) -> Result<Library, Error> {
    let root_path = super::path_helper::normalize(path)?;
    let lib_path = root_path.join(name);

    set_sources(&root_path);

    unsafe {
        Library::new(lib_path).map_err(|_| {
            Error::new(
                ErrorKind::NotFound,
                format!("Failed to read Steam installation path"),
            )
        })
    }
}
