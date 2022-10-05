use flt_rust_demo::*;
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};


// my_window with return value hwnd
fn my_window()  {
    let a = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    win.set_label("r2");
    let mut btn = button::Button::new(160, 200, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

    // input
    let mut inp = input::Input::new(160, 100, 80, 30, "Input");
    // set focus on input
    inp.take_focus();


    win.end();
    win.show();
    a.run().unwrap();
}

fn main() {

    // run my_window non blocking
    std::thread::spawn(|| {
        my_window();
    });

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    let devices = manager.get_device_list();

    //Filter to pickup events from the first keyboard only
    let keyboard = devices.keyboards.first().unwrap();
    manager.filter_devices(vec![keyboard.name.clone()]);
    //manager.unfilter_devices();

    println!("{:?}", devices);
    'outer: loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                _ => (),
            }
            println!("hello world");
            println!("{:?}", event);

            // activate the window with the title "r2"
            unsafe {
                winapi::um::winuser::SetForegroundWindow(winapi::um::winuser::FindWindowA(std::ptr::null(), "r2\0".as_ptr() as *const i8));
            }

        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    println!("Finishing");
}
