use winapi::shared::windef::HWND__;

pub fn get_hwnd_barcode_scanner() -> *mut HWND__ {
    let my_windows_hwnd = unsafe {
        winapi::um::winuser::FindWindowA(std::ptr::null(), "BarcodeScanner\0".as_ptr() as *const i8)
    };
    return my_windows_hwnd;
}