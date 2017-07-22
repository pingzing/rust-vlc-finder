use std::env;
use std::process::Command;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;

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
                user32::EnumWindows(Some(enumerate_callback), 0i64);
            }
        }
        _ => println!("Please come back with the file you want to open with VLC"),
    }
}

unsafe extern "system" fn enumerate_callback(
    hwnd: winapi::HWND, 
    _: winapi::LPARAM) 
    -> winapi::BOOL {        
        let title_length = user32::GetWindowTextLengthW(hwnd) + 1;
        let mut v: Vec<u16> = Vec::with_capacity(title_length  as usize);
        let read_len = user32::GetWindowTextW(hwnd, v.as_mut_ptr(), title_length);

        // Not all windows have titles, ignore those that don't
        if read_len > 0 {
            v.set_len((read_len) as usize);
            let string = String::from_utf16_lossy(&v);
            let wide_string: Vec<u16> = OsStr::new(string.as_str()).encode_wide().chain(once(0)).collect();
            if string.contains("VLC media player") {                                
                let window_handle = user32::FindWindowW(std::ptr::null_mut(), wide_string.as_ptr());
                user32::SetForegroundWindow(window_handle);
                user32::ShowWindow(window_handle, 9);
                return winapi::FALSE;
            }
        }        
        
        return winapi::TRUE;                                
}