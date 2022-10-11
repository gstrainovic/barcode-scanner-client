use std::sync::Arc;

use flt_rust_demo::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::app;
use fltk::{prelude::*, *};
use fltk::{
    button::*,
    group::{Group, Pack, Tabs},
    input::Input,
    menu::{Choice, MenuButton},
    output::Output,
};
use fltk_theme::{ThemeType, WidgetTheme};
use notify_rust::Notification;
use serde_json::json;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

// hello world function with reqwest and response with a object
#[tokio::main]
async fn loginfn() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://167.235.59.184:1337/api/auth/local")
        .json(&json!({
          "identifier": "g.strainovic@gmail.com",
          "password": "njM3&?HwtCe#GhV"
        }))
        .send()
        .await?;
    let body = res.json::<serde_json::Value>().await?;
    println!("{:?}", body);
    Ok(())
}



fn draw_gallery(w: i32 ,h: i32) {

    let mut tab = Tabs::new(10, 10,  w - 20, h - 20, "");

      let grp1 = Group::new(10, 35, w - 20, h - 45, "Tab1\t\t");
        let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
        pack.set_spacing(10);
        pack.end();
        let mut g1next = button::Button::new(w - 125, h - 50, 100, 30, "Weiter");
        let mut g1back = button::Button::new(25, h - 50, 100, 30, "Zurück");
      grp1.end();

      let grp2 = Group::new(10, 35, w - 30, h - 25, "Tab2\t\t");
        let mut col = group::Flex::default_fill().column();
        main_panel(&mut col);
        col.end();
        let mut g2next = button::Button::new(w - 125, h - 50, 100, 30, "Weiter");
        let mut g2back = button::Button::new(25, h - 50, 100, 30, "Zurück");
      grp2.end();
    
      let grp3 = Group::new(10, 35, w - 30, 450 - 25, "Tab3\t\t");
        let mut inp = input::Input::default()
            .with_size(320, 30)
            .center_of_parent();
        inp.set_trigger(enums::CallbackTrigger::EnterKey);
        inp.set_callback(|i| process_barcode(i));
        let mut g3next = button::Button::new(w - 125, h - 50, 100, 30, "Weiter");
        let mut g3back = button::Button::new(25, h - 50, 100, 30, "Zurück");
      grp3.end();

        g1back.deactivate();
        
        g1next.set_callback({
            let mut wiz_c = tab.clone();
            let mut grp2_c = grp2.clone();
            move |_| {
                wiz_c.set_value(&grp2_c);
            }
        });

        g2next.set_callback({
            let mut wiz_c = tab.clone();
            move |_| {
                wiz_c.set_value(&grp3);
            }
        });

        g2back.set_callback({
            let mut wiz_c = tab.clone();
            move |_| {
                wiz_c.set_value(&grp1);
            }
        });

        g3back.set_callback({
            let mut wiz_c = tab.clone();
            let mut grp2_c = grp2.clone();
            move |_| {
                wiz_c.set_value(&grp2_c);
            }
        });

        g3next.deactivate();

    tab.end();

    // let groups1 = [grp1, grp2, grp3];
    // let groups2 = [&grp1, &grp2, &grp3];

    // next and back buttons on the bottom for the tabs

    // next.set_callback(move | b | {
    //   let current_tab = groups1.iter().position(|x| x.visible()).unwrap();
    //   let grouplen = groups1.len() - 1 ;
      
    //   println!("current tab: {}", current_tab);
    //   println!("group len: {}", grouplen);
    //   println!("************");

    //   if current_tab < grouplen {
    //     let nw = groups1.get(current_tab + 1).unwrap();
    //     tab.set_value(nw);
    //   }

    //   let current_tab = groups1.iter().position(|x| x.visible()).unwrap();
    //   if current_tab == grouplen  {
    //     b.deactivate();
    //   }

    //   println!("current tab: {}", current_tab);
    //   println!("group len: {}", grouplen);
    //   println!("************");
    // });

    // back.set_callback(move | bb| {
    //   let current_tab = groups2.iter().position(|x| x.visible()).unwrap();
    //   let grouplen = groups2.len() - 1 ;
      
    //   println!("current tab: {}", current_tab);
    //   println!("group len: {}", grouplen);
    //   println!("************");

    //   if current_tab < grouplen {
    //     let nw2 = groups2.get(current_tab - 1).unwrap();
    //     tab.set_value(nw2);
    //   }

    //   let current_tab = groups2.iter().position(|x| x.visible()).unwrap();
    //   if current_tab == grouplen  {
    //     bb.deactivate();
    //   }

    //   println!("current tab: {}", current_tab);
    //   println!("group len: {}", grouplen);
    //   println!("************");
    // });



}

fn main() {

    let w = 640;
    let h = 480;

    // Create the application with ThemeType::Dark
    let a = app::App::default();
    
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let mut win = window::Window::default().with_size(w, h);
    win.set_label("BarcodeScanner");
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

    draw_gallery(w, h);

    win.end();
    win.show();

    std::thread::spawn(|| looper());

    a.run().unwrap();
}

fn buttons_panel(parent: &mut group::Flex) {
    frame::Frame::default();
    let mut w = frame::Frame::default().with_label("Bitte anmelden");
    w.set_label_size(20);

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Benutzername:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let username = input::Input::default();

        urow.set_size(&username, 180);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Passwort:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let password = input::Input::default();

        prow.set_size(&password, 180);
        prow.end();
    }

    let pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    {
        frame::Frame::default();
        let mut login = create_button("Anmelden");

        login.set_callback(|_| {
            let mut notif = Notification::new();
            notif.summary("Anmeldung");
            notif.show().unwrap();

            let resp = loginfn();
            println!("{:#?}", resp);
        });
        
        brow.set_size(&login, 180);
        // brow.set_size(&login, 80);
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

    // Takes a path
    let mut frame = frame::Frame::default();
    let mut myimage = image::SvgImage::load("gravurzeile.svg").unwrap();
    myimage.scale(200, 200, true, true);
    frame.set_image(Some(myimage));

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

fn create_button(caption: &str) -> button::ReturnButton {
    let mut btn = button::ReturnButton::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn.set_size(500, 30);
    btn
}

fn process_barcode(i: &mut input::Input) {
    // set focus to input field
    i.activate();
    let barcode = i.value();
    // remove value from input
    println!("Barcode: {}", barcode);
    let mut notif = Notification::new();
    notif.summary("Barcode");
    notif.body(&barcode);
    notif.show().unwrap();
    i.set_value("");
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

// #[tokio::main]
// async fn my_get() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

fn looper() {
    hide_console_window();

    // my_get().unwrap();

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
            }

            match event {
                RawEvent::KeyboardEvent(_, KeyId::Return, State::Released) => {
                    // activate the window current_active_window_hwnd again
                    unsafe {
                        winapi::um::winuser::ShowWindow(
                            my_windows_hwnd,
                            winapi::um::winuser::SW_MINIMIZE,
                        );
                        winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                        winapi::um::winuser::SetActiveWindow(switch_back_hwd);
                    }
                }

                _ => {}
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
