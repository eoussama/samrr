use std::ffi::CString;
use std::os::raw::c_char;

use libloading::Library;

use crate::helpers::interface;
use crate::utils::error;

type HSteamPipe = i32;
type HSteamUser = i32;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum EAccountType {
    Invalid = 0,
    Individual = 1,
    Multiseat = 2,
    GameServer = 3,
    AnonGameServer = 4,
    Pending = 5,
    ContentServer = 6,
    Clan = 7,
    Chat = 8,
    ConsoleUser = 9,
    AnonUser = 10,
    Max = 11,
}

struct SteamClient018 {
    vtable: *const SteamClient018VTable,
}

#[repr(C)]
struct SteamClient018VTable {
    create_steam_pipe: unsafe extern "C" fn() -> HSteamPipe,
    b_release_steam_pipe: unsafe extern "C" fn(HSteamPipe) -> bool,
    connect_to_global_user: unsafe extern "C" fn(HSteamPipe) -> HSteamUser,
    create_local_user: unsafe extern "C" fn(HSteamPipe, EAccountType) -> HSteamUser,
    release_user: unsafe extern "C" fn(HSteamPipe, HSteamUser),
    get_i_steam_user:
        unsafe extern "C" fn(HSteamUser, HSteamPipe, *const c_char) -> *const ISteamUserVTable,
    // get_i_steam_user: unsafe extern "C" fn(HSteamUser, HSteamPipe, *const c_char) -> *mut c_void,
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

#[repr(C)]
struct ISteamUser {
    vtable: *const ISteamUserVTable,
}

#[repr(C)]
struct ISteamUserVTable {
    get_h_steam_user: unsafe extern "C" fn() -> HSteamUser,
    // get_steam_id: unsafe extern "C" fn(*mut ISteamUser) -> u64,
    // get_personam_name: unsafe extern "C" fn(*mut ISteamUser) -> *const i8,
}

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

        let steam_user_012_name = CString::new("SteamUser012").unwrap();
        let get_i_steam_user = (*steam_client_018.vtable).get_i_steam_user;
        let steam_user_012 =
            get_i_steam_user(user, pipe, steam_user_012_name.as_ptr() as *const c_char);
        println!("[get_i_steam_user] {:?}", get_i_steam_user);
        println!("[steam_user_012] {:?}", steam_user_012);

        // // Get ISteamUser interface
        // let steam_user_012_name = CString::new("SteamUser012").unwrap();
        // println!("steam_user_012_name = {:?}", steam_user_012_name);

        // let steam_user_012_ptr = get_isteam_user(
        //     steam_user_handle,
        //     steam_pipe_handle,
        //     steam_user_012_name.as_ptr() as c_char,
        // );
        // println!("steam_user_012_ptr = {:?}", steam_user_012_ptr);

        // if steam_user_012_ptr.is_null() {
        //     panic!("[Error] Could not get ISteamUser interface");
        // }

        // let steam_user = &*(steam_user_012_ptr as *const ISteamUser);
        // let get_persona_name = (*steam_user.vtable).GetPersonaName;

        // // Fetch the user name
        // let persona_name_ptr = get_persona_name(steam_user as *const _ as *mut _);
        // let persona_name = CStr::from_ptr(persona_name_ptr).to_str().unwrap();

        // println!("Steam user name: {}", persona_name);

        // let create_interface: Symbol<
        //     unsafe extern "C" fn(version: *const i8, _arg: *mut std::ffi::c_void) -> *mut ISteamClient,
        // >;
    }

    Ok(())
}
