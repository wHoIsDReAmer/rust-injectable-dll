extern crate winapi;
extern crate sharedlib;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use sharedlib::{Lib, Func, Symbol};

use winapi::ctypes::c_int;

use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, UINT, TRUE};
use winapi::shared::ntdef::LPCWSTR;

fn to_utf16(str: &str) -> Vec<u16> {
    let vec: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    vec
}

pub fn message_box(title: &str, message: &str) {
    unsafe {
        let lib = Lib::new("user32.dll").unwrap();
        let symbol: Func<extern "C" fn(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT)
                                    -> c_int> = lib.find_func("MessageBoxW").unwrap();
        let mbw = symbol.get();
        mbw(0 as HWND,
            to_utf16(message).as_ptr(),
            to_utf16(title).as_ptr(),
            0);
    }
}


#[no_mangle]
pub extern "stdcall" fn DllMain(hinst: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOL {
    match reason {
        0 => {
        },
        1 => {
            message_box("hello", "world!!");
        },
        _ => {}
    };
    TRUE
}