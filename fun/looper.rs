use fltk::{
    input,
    menu::Choice,
    prelude::{MenuExt, WidgetExt},
};
use scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use std::sync::Arc;

use crate::{errors::Status, ERROR_STATUS};

pub fn looper(mut inp: input::Input, chce: Choice) {
    println!("Looper started");
    println!("Choice {}", chce.choice().unwrap().to_string());

    let mut switch_back_hwd = unsafe { winapi::um::winuser::GetForegroundWindow() };

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let keyboards = Arc::new(devices.keyboards);

    let keyboard = keyboards[chce.value() as usize].clone();

    manager.filter_devices(vec![keyboard.name.clone()]);

    loop {
        // handle events
        if let Some(event) = manager.get_event() {
            println!("Event: {:?}", event);

            let my_windows_hwnd = unsafe {
                winapi::um::winuser::FindWindowA(
                    std::ptr::null(),
                    "BarcodeScanner\0".as_ptr() as *const i8,
                )
            };

            let current_active_window_hwnd = unsafe { winapi::um::winuser::GetForegroundWindow() };

            if current_active_window_hwnd != my_windows_hwnd {
                switch_back_hwd = current_active_window_hwnd;
            }

            unsafe {
                winapi::um::winuser::ShowWindow(my_windows_hwnd, winapi::um::winuser::SW_MAXIMIZE);
                winapi::um::winuser::SetForegroundWindow(my_windows_hwnd);
                winapi::um::winuser::SetActiveWindow(my_windows_hwnd);
                let _ = inp.take_focus();
            }

            match event {
                RawEvent::KeyboardEvent(_, KeyId::Return, State::Released) => {
                    // activate the window current_active_window_hwnd again
                    // std::thread::sleep(std::time::Duration::from_millis(5000));
                    unsafe {
                        match ERROR_STATUS {
                            Status::Ok => {
                                winapi::um::winuser::ShowWindow(
                                    my_windows_hwnd,
                                    winapi::um::winuser::SW_MINIMIZE,
                                );
                                winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                                winapi::um::winuser::SetActiveWindow(switch_back_hwd);
                            }
                            _ => {}
                        }
                    }
                }

                _ => {}
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
