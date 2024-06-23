use libloading::{Library, Symbol};
use std::{ffi::CString, path::Path};

use super::env;
use super::error;
use super::path;

struct ISteamClient;
struct ISteamApps;
struct ISteamUser;

impl ISteamClient {
    fn create_steam_pipe(&self) -> i32 {
        1
    }
    fn connect_to_global_user(&self, _pipe: i32) -> i32 {
        1
    }
    fn get_isteam_apps(&self, _user: i32, _pipe: i32, _version: &str) -> Option<ISteamApps> {
        Some(ISteamApps)
    }
    fn get_isteam_user(&self, _user: i32, _pipe: i32, _version: &str) -> Option<ISteamUser> {
        Some(ISteamUser)
    }
}

fn set_sources(path: &Path) {
    let path_bin = path.join("bin");

    env::set_path(path);
    env::set_path(&path_bin);
}

pub fn export(lib: Library, name: &str) {
    let symbol = name.as_bytes();
    let create_interface: Symbol<
        unsafe extern "C" fn(version: *const i8, _arg: *mut std::ffi::c_void) -> *mut ISteamClient,
    >;

    unsafe {
        create_interface = lib
            .get(b"CreateInterface\0")
            .expect("Error reading the CreateInterface symbol from the Steam Client library.");
    }
    println!("create_interface = {:?}", create_interface);

    let create_interface = unsafe {
        create_interface(
            CString::new("SteamClient019").unwrap().as_ptr(),
            std::ptr::null_mut(),
        )
    };
    println!("create_interface = {:?}", create_interface);

    let steam_client = unsafe { &*create_interface };

    let pipe = steam_client.create_steam_pipe();
    println!("pipe = {:?}", pipe);

    let user = steam_client.connect_to_global_user(pipe);
    println!("user = {:?}", user);

    // println!("func = {:?}", func);
    // let result = func("SteamClient019", std::ptr::null_mut());
    // println!("result = {:?}", result);
}

pub fn load(path: String, name: &str) -> Result<Library, error::Error> {
    let root_path = path::normalize(path)?;
    let lib_path = root_path.join(name);

    set_sources(&root_path);

    unsafe { Library::new(lib_path).map_err(|_| error::Error::LibraryNotFound) }
}
