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
    get_i_steam_user: unsafe extern "C" fn(HSteamUser, HSteamPipe, *const c_char) -> *const ISteamUserVTable,
    // get_i_steam_user:
    //     unsafe extern "C" fn(HSteamUser, HSteamPipe, *const c_char) -> *const SteamUser012,
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

struct SteamUser012 {
    vtable: *const ISteamUserVTable,
}

#[repr(C)]
struct ISteamUserVTable {
    get_h_steam_user: unsafe extern "C" fn() -> HSteamUser,
    // b_logged_on: unsafe extern "C" fn() -> bool,
    // get_steam_id: unsafe extern "C" fn() -> CSteamID,
    // get_steam_id: unsafe extern "C" fn(*mut ISteamUser) -> u64,
    // get_personam_name: unsafe extern "C" fn(*mut ISteamUser) -> *const i8,
    // GetHSteamUser: unsafe extern "C" fn() -> HSteamUser,
    // LoggedOn: unsafe extern "C" fn() -> HSteamUser,
    // GetSteamID: unsafe extern "C" fn() -> HSteamUser,
    // InitiateGameConnection: unsafe extern "C" fn() -> HSteamUser,    
    // TerminateGameConnection: unsafe extern "C" fn() -> HSteamUser,
    // TrackAppUsageEvent: unsafe extern "C" fn() -> HSteamUser,
    // GetUserDataFolder: unsafe extern "C" fn() -> HSteamUser,
    // StartVoiceRecording: unsafe extern "C" fn() -> HSteamUser,
    // StopVoiceRecording: unsafe extern "C" fn() -> HSteamUser,
    // GetCompressedVoice: unsafe extern "C" fn() -> HSteamUser,    
    // DecompressVoice: unsafe extern "C" fn() -> HSteamUser,
    // GetAuthSessionTicket: unsafe extern "C" fn() -> HSteamUser,
    // BeginAuthSession: unsafe extern "C" fn() -> HSteamUser,
    // EndAuthSession: unsafe extern "C" fn() -> HSteamUser,
    // CancelAuthTicket: unsafe extern "C" fn() -> HSteamUser,
    // UserHasLicenseForApp: unsafe extern "C" fn() -> HSteamUser, 
}

// #[repr(C)]
// struct ISteamUserVTable {
//     get_h_steam_user: unsafe extern "C" fn() -> HSteamUser,
//     b_logged_on: unsafe extern "C" fn() -> bool
// }

pub fn init(lib: &Library) -> Result<(), error::Error> {
    let i_steam_client_018_ptr = interface::create::<ISteamClient>(lib, "SteamClient018")?;
    dbg!(i_steam_client_018_ptr);

    unsafe {
        let steam_client_018 = &*(i_steam_client_018_ptr as *const SteamClient018);
        dbg!(steam_client_018.vtable);

        let create_steam_pipe = (*steam_client_018.vtable).create_steam_pipe;
        let pipe = create_steam_pipe();
        dbg!(create_steam_pipe);
        dbg!(pipe);

        let connect_to_global_user = (*steam_client_018.vtable).connect_to_global_user;
        let user = connect_to_global_user(pipe);
        dbg!(connect_to_global_user);
        dbg!(user);

        let steam_user_012_name = CString::new("SteamUser012").unwrap();
        let get_i_steam_user = (*steam_client_018.vtable).get_i_steam_user;
        let steam_user_012_ptr = get_i_steam_user(user, pipe, steam_user_012_name.as_ptr() as *const c_char);
        dbg!(get_i_steam_user);
        dbg!(steam_user_012_ptr);

        let steam_user_012 = &*(steam_user_012_ptr as *const SteamUser012);
        let get_h_steam_user = (*steam_user_012.vtable).get_h_steam_user;
        dbg!(get_h_steam_user);
        dbg!(get_h_steam_user());
    }

    Ok(())
}
