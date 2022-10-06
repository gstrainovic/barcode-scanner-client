use std::sync::Arc;

use flt_rust_demo::{RawInputManager, DeviceType, RawEvent, KeyId, State};
// use flt_rust_demo::*;
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

fn my_window() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    win.set_label("r2");

    // switch theme to dark
    let theme = WidgetTheme::new(ThemeType::Dark);
    theme.apply();

    let mut inp = input::Input::default()
        .with_size(160, 30)
        .center_of_parent();
    inp.set_trigger(enums::CallbackTrigger::EnterKey);
    inp.set_callback(|i| println!("hello {}", i.value()));

    win.end();
    win.show();
    a.run().unwrap();}

fn main() {
    // run my_window non blocking
    std::thread::spawn(|| {
        my_window();
    });

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();

    //Filter to pickup events from the first keyboard only
    let keyboard = devices.keyboards.first().unwrap();
    manager.filter_devices(vec![keyboard.name.clone()]);
    //manager.unfilter_devices();

    println!("{:?}", devices);

    // list of characters
    let mut switch_back_hwd = unsafe {
        winapi::um::winuser::GetForegroundWindow()
    };


    loop {
        if let Some(event) = manager.get_event() {
            // get HWND from window with the title 2
            let my_windows_hwnd = unsafe { winapi::um::winuser::FindWindowA(std::ptr::null(), "r2\0".as_ptr() as *const i8) };

            let current_active_window_hwnd = unsafe {
                winapi::um::winuser::GetForegroundWindow()
            };

            if current_active_window_hwnd != my_windows_hwnd {
                switch_back_hwd = current_active_window_hwnd;
            }


            // activate the window with the title "r2"
            unsafe {
                winapi::um::winuser::SetForegroundWindow(winapi::um::winuser::FindWindowA(std::ptr::null(), "r2\0".as_ptr() as *const i8));
            }

            match event {

              // RawEvent::KeyboardEvent(_, KeyId::One , State::Pressed) => { chars.push('1'); }
              // RawEvent::KeyboardEvent(_, KeyId::Two , State::Pressed) => { chars.push('2'); }
              // RawEvent::KeyboardEvent(_, KeyId::Three , State::Pressed) => { chars.push('3'); }
              // RawEvent::KeyboardEvent(_, KeyId::Four , State::Pressed) => { chars.push('4'); }
              // RawEvent::KeyboardEvent(_, KeyId::Five , State::Pressed) => { chars.push('5'); }
              // RawEvent::KeyboardEvent(_, KeyId::Six , State::Pressed) => { chars.push('6'); }
              // RawEvent::KeyboardEvent(_, KeyId::Seven , State::Pressed) => { chars.push('7'); }
              // RawEvent::KeyboardEvent(_, KeyId::Eight , State::Pressed) => { chars.push('8'); }
              // RawEvent::KeyboardEvent(_, KeyId::Nine , State::Pressed) => { chars.push('9'); }
              
              // RawEvent::KeyboardEvent(_, KeyId::A , State::Pressed) => { chars.push('a'); }
              // RawEvent::KeyboardEvent(_, KeyId::B , State::Pressed) => { chars.push('b'); }
              // RawEvent::KeyboardEvent(_, KeyId::C , State::Pressed) => { chars.push('c'); }
              // RawEvent::KeyboardEvent(_, KeyId::D , State::Pressed) => { chars.push('d'); }
              // RawEvent::KeyboardEvent(_, KeyId::E , State::Pressed) => { chars.push('e'); }
              // RawEvent::KeyboardEvent(_, KeyId::F , State::Pressed) => { chars.push('f'); }
              // RawEvent::KeyboardEvent(_, KeyId::G , State::Pressed) => { chars.push('g'); }
              // RawEvent::KeyboardEvent(_, KeyId::H , State::Pressed) => { chars.push('h'); }
              // RawEvent::KeyboardEvent(_, KeyId::I , State::Pressed) => { chars.push('i'); }
              // RawEvent::KeyboardEvent(_, KeyId::J , State::Pressed) => { chars.push('j'); }
              // RawEvent::KeyboardEvent(_, KeyId::K , State::Pressed) => { chars.push('k'); }
              // RawEvent::KeyboardEvent(_, KeyId::L , State::Pressed) => { chars.push('l'); }
              // RawEvent::KeyboardEvent(_, KeyId::M , State::Pressed) => { chars.push('m'); }
              // RawEvent::KeyboardEvent(_, KeyId::N , State::Pressed) => { chars.push('n'); }
              // RawEvent::KeyboardEvent(_, KeyId::O , State::Pressed) => { chars.push('o'); }
              // RawEvent::KeyboardEvent(_, KeyId::P , State::Pressed) => { chars.push('p'); }
              // RawEvent::KeyboardEvent(_, KeyId::Q , State::Pressed) => { chars.push('q'); }
              // RawEvent::KeyboardEvent(_, KeyId::R , State::Pressed) => { chars.push('r'); }
              // RawEvent::KeyboardEvent(_, KeyId::S , State::Pressed) => { chars.push('s'); }
              // RawEvent::KeyboardEvent(_, KeyId::T , State::Pressed) => { chars.push('t'); }
              // RawEvent::KeyboardEvent(_, KeyId::U , State::Pressed) => { chars.push('u'); }
              // RawEvent::KeyboardEvent(_, KeyId::V , State::Pressed) => { chars.push('v'); }
              // RawEvent::KeyboardEvent(_, KeyId::W , State::Pressed) => { chars.push('w'); }
              // RawEvent::KeyboardEvent(_, KeyId::X , State::Pressed) => { chars.push('x'); }
              // RawEvent::KeyboardEvent(_, KeyId::Y , State::Pressed) => { chars.push('y'); }
              // RawEvent::KeyboardEvent(_, KeyId::Z , State::Pressed) => { chars.push('z'); }
              
              // RawEvent::KeyboardEvent(_, KeyId::Space , State::Pressed) => { chars.push(' '); }
              // RawEvent::KeyboardEvent(_, KeyId::Subtract , State::Pressed) => { chars.push('-'); }
              // RawEvent::KeyboardEvent(_, KeyId::Multiply , State::Pressed) => { chars.push('*'); }
              // RawEvent::KeyboardEvent(_, KeyId::Separator , State::Pressed) => { chars.push(','); }
              // RawEvent::KeyboardEvent(_, KeyId::Decimal , State::Pressed) => { chars.push('.'); }
              // RawEvent::KeyboardEvent(_, KeyId::Divide , State::Pressed) => { chars.push('/'); }
              // RawEvent::KeyboardEvent(_, KeyId::BackTick , State::Pressed) => { chars.push('`'); }
              // RawEvent::KeyboardEvent(_, KeyId::BackSlash , State::Pressed) => { chars.push('\\'); }
              // RawEvent::KeyboardEvent(_, KeyId::ForwardSlash , State::Pressed) => { chars.push('/'); }
              // RawEvent::KeyboardEvent(_, KeyId::Plus , State::Pressed) => { chars.push('+'); }
              // RawEvent::KeyboardEvent(_, KeyId::Minus , State::Pressed) => { chars.push('-'); }
              // RawEvent::KeyboardEvent(_, KeyId::Comma , State::Pressed) => { chars.push(','); }
              // RawEvent::KeyboardEvent(_, KeyId::LeftSquareBracket , State::Pressed) => { chars.push('['); }
              // RawEvent::KeyboardEvent(_, KeyId::RightSquareBracket , State::Pressed) => { chars.push(']'); }
              // RawEvent::KeyboardEvent(_, KeyId::SemiColon , State::Pressed) => { chars.push(';'); }
              // RawEvent::KeyboardEvent(_, KeyId::Apostrophe, State::Pressed) => { chars.push('\''); }
              RawEvent::KeyboardEvent(_, KeyId::Return , State::Released) => { 

                  // // string from chars
                  // let mut string = String::new();
                  // for c in chars.iter() {
                  //     string.push(*c);
                  // }

                  // // print string
                  // println!("{}", string);

                  // // clear chars and string
                  // chars.clear();
                  // string.clear();

                  // activate the window current_active_window_hwnd again
                  unsafe {
                      winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                  }

                  // clear current_active_window_hwnd

               }
              

              _ => {}

            }

        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
      }
    }

    