use flt_rust_demo::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::{prelude::*, *};
use fltk_theme::{ThemeType, WidgetTheme};
use fltk::{app, enums::FrameType};
use notify_rust::Notification;
use winput::{Vk, Action};
use winput::message_loop;

#[cfg(target_os = "windows")]
mod systray;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

fn login_demo() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(640, 480);
    let mut col = group::Flex::default_fill().column();
    main_panel(&mut col);
    col.end();
    win.resizable(&col);
    win.set_color(enums::Color::from_rgb(250, 250, 250));
    win.end();
    win.show();
    // hide the window
    // win.hide();
    win.size_range(600, 400, 0, 0);
    a.run().unwrap();
}

fn buttons_panel(parent: &mut group::Flex) {
    frame::Frame::default();
    let w = frame::Frame::default().with_label("Welcome to Flex Login");

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Username:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let username = input::Input::default();

        urow.set_size(&username, 180);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Password:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let password = input::Input::default();

        prow.set_size(&password, 180);
        prow.end();
    }

    let pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    {
        frame::Frame::default();
        let reg = create_button("Register");
        let login = create_button("Login");

        brow.set_size(&reg, 80);
        brow.set_size(&login, 80);
        brow.end();
    }

    let b = frame::Frame::default();

    frame::Frame::default();

    parent.set_size(&w, 60);
    parent.set_size(&urow, 30);
    parent.set_size(&prow, 30);
    parent.set_size(&pad, 1);
    parent.set_size(&brow, 30);
    parent.set_size(&b, 30);
}

fn middle_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut frame = frame::Frame::default().with_label("Image");
    frame.set_frame(enums::FrameType::BorderBox);
    frame.set_color(enums::Color::from_rgb(0, 200, 0));
    let spacer = frame::Frame::default();

    let mut bp = group::Flex::default().column();
    buttons_panel(&mut bp);
    bp.end();

    frame::Frame::default();

    parent.set_size(&frame, 200);
    parent.set_size(&spacer, 10);
    parent.set_size(&bp, 300);
}

fn main_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut mp = group::Flex::default().row();
    middle_panel(&mut mp);
    mp.end();

    frame::Frame::default();

    parent.set_size(&mp, 200);
}

fn create_button(caption: &str) -> button::Button {
    let mut btn = button::Button::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn
}


fn process_barcode(barcode: &str) {
    println!("Barcode: {}", barcode);
    let mut notif = Notification::new();
    notif.summary("Barcode");
    notif.body(&barcode);
    notif.show().unwrap();
}

fn main() {

    hide_console_window();

    let a = app::App::default();
    let mut win = window::Window::default().with_size(800, 600);
    win.set_label("r2");

    // switch theme to dark
    let theme = WidgetTheme::new(ThemeType::Dark);
    theme.apply();

    let mut inp = input::Input::default()
        .with_size(320, 30)
        .center_of_parent();
    inp.set_trigger(enums::CallbackTrigger::EnterKey);
    inp.set_callback(|i| process_barcode(&i.value()));

    win.end();
    win.show();
    // do you really want do close the window?
    win.set_callback(|w| {
        let choice = dialog::choice2_default("Barcodescanner beenden?", "Nein", "Ja", "Abbruch");
        println!("{:?}", choice);
        if choice == Some(1) {
              let mut notif = Notification::new();
              notif.summary("Barcodescanner beendet");
              notif.show().unwrap();
            w.hide();
        }
    });

    let my_looper_handle = std::thread::spawn(|| looper());

    a.run().unwrap();
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}


fn looper() {

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();

    //Filter to pickup events from the first keyboard only
    let keyboard = devices.keyboards.first().unwrap();
    manager.filter_devices(vec![keyboard.name.clone()]);
    //manager.unfilter_devices();

    println!("{:?}", devices);

    // list of characters
    let mut switch_back_hwd = unsafe { winapi::um::winuser::GetForegroundWindow() };

    loop {
        // handle events
        if let Some(event) = manager.get_event() {
            let my_windows_hwnd = unsafe {
                winapi::um::winuser::FindWindowA(std::ptr::null(), "r2\0".as_ptr() as *const i8)
            };

            let current_active_window_hwnd = unsafe { winapi::um::winuser::GetForegroundWindow() };

            if current_active_window_hwnd != my_windows_hwnd {
                switch_back_hwd = current_active_window_hwnd;
            }

            unsafe {  
              // maximize window
              winapi::um::winuser::ShowWindow(my_windows_hwnd, winapi::um::winuser::SW_MAXIMIZE);
              winapi::um::winuser::SetForegroundWindow(my_windows_hwnd);
              winapi::um::winuser::SetActiveWindow(my_windows_hwnd);
            }

            match event {
                RawEvent::KeyboardEvent(_, KeyId::Return, State::Released) => {
                    // activate the window current_active_window_hwnd again
                    unsafe {
                        winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                    }
                }

                _ => {}
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
