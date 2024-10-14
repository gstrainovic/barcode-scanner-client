#[cfg(windows)] extern crate winapi;
use std::io::Error;


#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}


#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    // println!("{}", msg);
    Ok(())
}


fn activate( ) {
    #[cfg(windows)]
    {
        use winapi::um::winuser::{FindWindowW, ShowWindow, SW_RESTORE};
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr::null_mut;
        let wide: Vec<u16> = OsStr::new("r1").encode_wide().chain(once(0)).collect();
        let hwnd = unsafe {
            FindWindowW(null_mut(), wide.as_ptr())
        };
        
        // print hwnd to console
        let string_hwnd = format!("{:?}", hwnd); 
        if hwnd != null_mut() {
            unsafe {
                ShowWindow(hwnd, SW_RESTORE);
            }
        }
      }

    
}