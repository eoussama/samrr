use std::env;
use std::path::Path;

use libloading::Library;

pub fn load_library(dir_path: &str, dll_name: &str) -> Result<Library, libloading::Error> {
    let bin_path = Path::new(dir_path).join("bin");

    if let Some(mut current_path) = env::var_os("PATH") {
        current_path.push(";");
        current_path.push(r"D:\games\Steam");
        current_path.push(";");
        current_path.push(r"D:\games\Steam\bin");
        env::set_var("PATH", current_path);

        match env::var_os("PATH") {
            Some(value) => println!("current_path = {:?}", value),
            None => println!("current_path err"),
        }
    }

    let path_str = String::from(dir_path) + "/" + dll_name;

    println!("path_str = {:?}", path_str);
    unsafe { Library::new(r"D:\games\Steam\steamclient64.dll") }
}
