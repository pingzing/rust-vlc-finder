use std::ffi::CString;
use std::env;
use std::process::Command;

extern crate winapi;
extern crate user32;


fn main() {

    match env::args().nth(1) {
        Some(arg) => {
            Command::new("explorer")
                .arg(arg)
                .spawn()
                .expect("couldn't open file???")
                .wait()
                .expect("child didn't terminate???");
            unsafe {
                user32::EnumWindows(Some(enum_windows_callback), 0i64);
            }
        }
        _ => println!("Please come back with the file you want to open with VLC"),
    }
}

extern "system" fn enum_windows_callback(
    hwnd: winapi::HWND,
    _: winapi::LPARAM,
) -> winapi::minwindef::BOOL {
    unsafe {
        let len = user32::GetWindowTextLengthA(hwnd);
        let mut v: Vec<u16> = Vec::with_capacity(len as usize);
        let read_len = user32::GetWindowTextW(hwnd as winapi::HWND, v.as_mut_ptr(), 255);
        v.set_len(read_len as usize);
        let string = String::from_utf16_lossy(&v);

        if string.contains("VLC media player") {
            let window_name = CString::new(string).unwrap();
            let window_handle = user32::FindWindowA(std::ptr::null_mut(), window_name.as_ptr());
            user32::SetForegroundWindow(window_handle);
            user32::ShowWindow(window_handle, 9);
            0
        } else {
            1
        }
    }
}
