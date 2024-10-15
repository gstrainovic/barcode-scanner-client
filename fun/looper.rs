use fltk::{
    input,
    menu::Choice,
    prelude::{MenuExt, WidgetExt},
};
use scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use std::sync::Arc;

use crate::{errors::Status, ERROR_STATUS};
use crate::sync::sync;

pub fn looper(mut inp: input::Input, chce: Choice, rolle: String, jwt: String) {
    sync(jwt); // einmalig synchronisieren von sqlite

    let mut switch_back_hwd = unsafe { winapi::um::winuser::GetForegroundWindow() };

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let keyboards = Arc::new(devices.keyboards);

    let keyboard = keyboards[chce.value() as usize].clone();

    manager.filter_devices(vec![keyboard.name.clone()]);

    let my_windows_hwnd = unsafe {
        winapi::um::winuser::FindWindowA(
            std::ptr::null(),
            "BarcodeScanner\0".as_ptr() as *const i8,
        )
    };

    loop {
        // handle events
        if let Some(event) = manager.get_event() {
            // println!("Event: {:?}", event);
            // add charachter from event to barcode_string

            let current_active_window_hwnd = unsafe { winapi::um::winuser::GetForegroundWindow() };
            if current_active_window_hwnd != my_windows_hwnd {
                switch_back_hwd = current_active_window_hwnd;
            }

            unsafe {
                winapi::um::winuser::ShowWindow(my_windows_hwnd, winapi::um::winuser::SW_MAXIMIZE);
                winapi::um::winuser::SetForegroundWindow(my_windows_hwnd);
                winapi::um::winuser::SetActiveWindow(my_windows_hwnd);
                winapi::um::winuser::SetFocus(my_windows_hwnd);
                let _ = inp.take_focus();
            }

            match event {
                RawEvent::KeyboardEvent(_, KeyId::Return, State::Released) => {
                    unsafe {
                    // activate the window current_active_window_hwnd again
                        match ERROR_STATUS {
                            Status::Ok => {
                                if rolle == "Produktion" {
                                    winapi::um::winuser::ShowWindow(
                                        my_windows_hwnd,
                                        winapi::um::winuser::SW_MINIMIZE,
                                    );
                                    winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                                    winapi::um::winuser::SetActiveWindow(switch_back_hwd);
                                    winapi::um::winuser::SetFocus(switch_back_hwd);
                                }
                            }
                            _ => {}
                        }
                    }
                }

                _ => {
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
            let has_focus_my_windows_hwnd = unsafe {
                winapi::um::winuser::GetForegroundWindow() == my_windows_hwnd
            };
            let is_a_mxg_box_open = unsafe {
                winapi::um::winuser::GetForegroundWindow()
                    == winapi::um::winuser::GetLastActivePopup(my_windows_hwnd)
            };
            // minimize the window if it has no focus, expect if there is an msgbox
            if !has_focus_my_windows_hwnd && !is_a_mxg_box_open {
                unsafe {
                    winapi::um::winuser::ShowWindow(
                        my_windows_hwnd,
                        winapi::um::winuser::SW_MINIMIZE,
                    );
                }
            }


        }
    }
}
