use std::path::Path;

use libloading::{Library, Symbol};

use super::env;
use super::error;
use super::path;

fn set_sources(path: &Path) {
    let path_bin = path.join("bin");

    env::set_path(path);
    env::set_path(&path_bin);
}

pub fn load(path: String, name: &str) -> Result<Library, error::Error> {
    let root_path = path::normalize(path)?;
    let lib_path = root_path.join(name);

    set_sources(&root_path);

    unsafe { Library::new(lib_path).map_err(|_| error::Error::LibraryNotFound) }
}

pub fn export<'a, T>(lib: &'a Library, symbol: &'a [u8]) -> Result<Symbol<'a, T>, error::Error> {
    unsafe { lib.get(symbol).map_err(|_| error::Error::FunctionNotFound) }
}
