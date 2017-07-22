use std::ffi::CString;

extern crate winapi;
extern crate user32;

fn main() {
    
    unsafe {      
        user32::EnumWindows(Some(enumerate_callback), 0i64);        
    }
}

unsafe extern "system" fn enumerate_callback(
    hwnd: winapi::HWND, 
    lparam: winapi::LPARAM) 
    -> winapi::BOOL {    
        let title_length = user32::GetWindowTextLengthW(hwnd) + 1;
        let mut v: Vec<u16> = Vec::with_capacity(title_length  as usize);
        let read_len = user32::GetWindowTextW(hwnd, v.as_mut_ptr(), title_length);

        // Not all windows have titles, ignore those that don't
        if read_len > 0 {
            v.set_len((read_len) as usize);
            let string = String::from_utf16_lossy(&v);            
            if string.contains("VLC media player") {
                let window_name = CString::new(string).unwrap();    
                let window_handle = user32::FindWindowA(std::ptr::null_mut(), window_name.as_ptr());
                user32::SetForegroundWindow(window_handle);
                user32::ShowWindow(window_handle, 9);
                return winapi::FALSE;
            }
        }        
        
        return winapi::TRUE;                                
}