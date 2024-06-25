use std::ffi::CString;
use std::os::raw::{c_char, c_void};

use libloading::{Library, Symbol};

use crate::helpers::library;
use crate::utils::error;

type CreateInterfaceFn<T> = unsafe extern "C" fn(c_char, *mut c_void) -> *mut T;

fn export_create_interface<T>(lib: &Library) -> Result<Symbol<CreateInterfaceFn<T>>, error::Error> {
    let symbol = b"CreateInterface";
    let create_interface = library::export::<CreateInterfaceFn<T>>(lib, symbol);

    create_interface
}

pub fn create<T>(lib: &Library, version: &str) -> Result<*mut T, error::Error> {
    let create_interface = export_create_interface::<T>(lib)?;

    let interface_name = CString::new(version).unwrap();
    let interface_ptr =
        unsafe { create_interface(interface_name.as_ptr() as c_char, std::ptr::null_mut()) };

    Ok(interface_ptr)
}
