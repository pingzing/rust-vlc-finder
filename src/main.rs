use std::ffi::CString;

extern crate winapi;
extern crate user32;

fn main() {
    let window_name = CString::new("VLC media player").unwrap();
    let window_handle;
    unsafe {
        window_handle = user32::FindWindowA(std::ptr::null_mut(), window_name.as_ptr());        
        user32::SetForegroundWindow(window_handle);
        user32::ShowWindow(window_handle, 9);
    }
}
