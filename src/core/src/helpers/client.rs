use libloading::Library;

use crate::helpers::library;
use crate::helpers::steam;
use crate::utils::error;
use crate::wrappers::i_steam_client_018;

fn load_steam_client() -> Result<Library, error::Error> {
    let lib_path = steam::get_path()?;
    let lib_name = "steamclient64.dll";

    let lib = library::load(lib_path, lib_name);

    lib
}

pub fn load() -> Result<bool, error::Error> {
    let lib = load_steam_client()?;
    let steam_client = i_steam_client_018::init(&lib);

    // // Create a Steam pipe
    // let steam_pipe_handle = create_steam_pipe();
    // println!("steam_pipe_handle = {:?}", steam_pipe_handle);

    // // Connect to the global user
    // let steam_user_handle = connect_to_global_user(steam_pipe_handle);
    // println!("steam_user_handle = {:?}", steam_user_handle);

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

    Ok(true)
}
