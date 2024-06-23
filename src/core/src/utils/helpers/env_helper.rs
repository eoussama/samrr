use std::env;
use std::path::Path;

pub fn set_path(fragment: &Path) {
    if let Some(mut path) = env::var_os("PATH") {
        path.push(";");
        path.push(fragment);

        env::set_var("PATH", path);
    }
}
