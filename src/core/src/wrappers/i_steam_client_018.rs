use std::os::raw::{c_char, c_void};

use libloading::Library;

use crate::helpers::interface;
use crate::utils::error;

type HSteamPipe = i32;
type HSteamUser = i32;

#[repr(C)]
struct SteamClient018 {
    vtable: *const SteamClient018VTable,
}

#[repr(C)]
struct SteamClient018VTable {
    create_steam_pipe: unsafe extern "C" fn() -> HSteamPipe,
    b_release_steam_pipe: unsafe extern "C" fn(HSteamPipe) -> bool,
    connect_to_global_user: unsafe extern "C" fn(HSteamPipe) -> HSteamUser,
    // get_steam_user_interface: unsafe extern "C" fn(
    //     user: HSteamUser,
    //     pipe: HSteamPipe,
    //     interface_name: c_char,
    // ) -> *mut c_void,
}

struct ISteamClient;

// struct ISteamApps;
// struct ISteamUser;

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

pub fn init(lib: &Library) -> Result<(), error::Error> {
    let i_steam_client_018_ptr = interface::create::<ISteamClient>(lib, "SteamClient018")?;
    println!("[i_steam_client_018_ptr] {:?}", i_steam_client_018_ptr);

    unsafe {
        let steam_client_018 = &*(i_steam_client_018_ptr as *const SteamClient018);

        let create_steam_pipe = (*steam_client_018.vtable).create_steam_pipe;
        let pipe = create_steam_pipe();
        println!("[create_steam_pipe] {:?}", create_steam_pipe);
        println!("[pipe] {:?}", pipe);

        let connect_to_global_user = (*steam_client_018.vtable).connect_to_global_user;
        let user = connect_to_global_user(pipe);
        println!("[connect_to_global_user] {:?}", connect_to_global_user);
        println!("[user] {:?}", user);
    }

    Ok(())
}
