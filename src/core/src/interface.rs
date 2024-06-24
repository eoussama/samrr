use std::ffi::CString;
use std::os::raw::{c_char, c_void};

use libloading::{Library, Symbol};

use crate::library;

use super::error;

type HSteamPipe = i32;
type HSteamUser = i32;
pub type CreateInterfaceFn<T> = unsafe extern "C" fn(c_char, *mut c_void) -> *mut T;

struct ISteamClient;
// struct ISteamApps;
struct ISteamUser;

// impl ISteamClient {
//     fn create_steam_pipe(&self) -> i32 {
//         1
//     }
//     fn connect_to_global_user(&self, _pipe: i32) -> i32 {
//         1
//     }
//     fn get_isteam_apps(&self, _user: i32, _pipe: i32, _version: &str) -> Option<ISteamApps> {
//         Some(ISteamApps)
//     }
//     fn get_isteam_user(&self, _user: i32, _pipe: i32, _version: &str) -> Option<ISteamUser1> {
//         Some(ISteamUser1)
//     }
// }

#[repr(C)]
struct SteamClient018 {
    vtable: *const SteamClient018VTable,
}

#[repr(C)]
struct SteamClient018VTable {
    create_steam_pipe: unsafe extern "C" fn() -> HSteamPipe,
    connect_to_global_user: unsafe extern "C" fn(pipe: HSteamPipe) -> HSteamUser,
    get_steam_user_interface: unsafe extern "C" fn(
        user: HSteamUser,
        pipe: HSteamPipe,
        interface_name: c_char,
    ) -> *mut c_void,
}

// #[repr(C)]
// struct ISteamUser {
//     vtable: *const ISteamUserVTable,
// }

// #[repr(C)]
// struct ISteamUserVTable {
//     GetSteamID: unsafe extern "C" fn(this: *mut ISteamUser) -> u64,
//     GetPersonaName: unsafe extern "C" fn(this: *mut ISteamUser) -> *const i8,
//     // Other functions can be added here
// }

fn export_create_interface<T>(lib: &Library) -> Result<Symbol<CreateInterfaceFn<T>>, error::Error> {
    let symbol = b"CreateInterface";
    let create_interface = library::export::<CreateInterfaceFn<T>>(lib, symbol);

    create_interface
}

fn create_interface<T>(lib: &Library, version: &str) -> Result<*mut T, error::Error> {
    let create = export_create_interface::<T>(lib)?;

    let interface_name = CString::new(version).unwrap();
    let interface_ptr = unsafe { create(interface_name.as_ptr() as c_char, std::ptr::null_mut()) };

    Ok(interface_ptr)
}

pub fn export_i_steam_client_018(lib: &Library) -> Result<(), error::Error> {
    let i_steam_client_018_ptr = create_interface::<ISteamClient>(lib, "SteamClient018")?;

    unsafe {
        let steam_client_018 = &*(i_steam_client_018_ptr as *const SteamClient018);
        let create_steam_pipe = (*steam_client_018.vtable).create_steam_pipe;
        let connect_to_global_user = (*steam_client_018.vtable).connect_to_global_user;

        println!("[i_steam_client_018_ptr] {:?}", i_steam_client_018_ptr);
        println!("[create_steam_pipe] {:?} = {:?}", create_steam_pipe, create_steam_pipe());
        println!("[connect_to_global_user] {:?} = {:?}", connect_to_global_user, connect_to_global_user(create_steam_pipe()));
    }

    Ok(())
}
