use std::sync::Arc;

use flt_rust_demo::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::{
    app, button, group,
    group::{Group, Pack, Tabs},
    window,
};
use fltk::{prelude::*, *};
use fltk_theme::{ThemeType, WidgetTheme};
use notify_rust::Notification;
use serde_json::{json, Value, Map};
use serde::Deserialize;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

#[derive(Deserialize, Debug)]
struct myjson {
    jwt: Option<String>,
    error: Map<String, Value>
  }

#[tokio::main]
async fn loginfn(user: String, pass: String) -> Result<(myjson), reqwest::Error> {

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post("http://167.235.59.184:1337/api/auth/local")
        .json(&json!({
          "identifier": user, //"g.strainovic@gmail.com",
          "password": pass // "njM3&?HwtCe#GhV"
        }))
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;

    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}


fn main() {
    hide_console_window();
    find_device();

    let w = 640;
    let h = 480;

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

    let wizard = group::Wizard::default().with_size(w, h);

    let grp1 = group::Group::default().size_of(&wizard);
    let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
    pack.set_spacing(10);
    pack.end();

    let col = group::Flex::default_fill().column();

    frame::Frame::default();

    let mut mp = group::Flex::default().row();

    frame::Frame::default();

    // Takes a path
    let mut frame = frame::Frame::default();
    let mut myimage = image::SvgImage::load("gravurzeile.svg").unwrap();
    myimage.scale(200, 200, true, true);
    frame.set_image(Some(myimage));

    let spacer = frame::Frame::default();

    let mut bp = group::Flex::default().column();

    frame::Frame::default();
    let mut wf = frame::Frame::default().with_label("Bitte anmelden");
    wf.set_label_size(20);

    let username = input::Input::default();
    let password = input::SecretInput::default();

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Benutzername:")
            .with_align(enums::Align::Inside | enums::Align::Right);

        urow.set_size(&username, 180);
        urow.add(&username);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Passwort:")
            .with_align(enums::Align::Inside | enums::Align::Right);

        prow.set_size(&password, 180);
        prow.add(&password);
        prow.end();
    }


    let pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    frame::Frame::default();
    let mut login = create_button("Anmelden");
    brow.set_size(&login, 180);
    brow.end();

    let b = frame::Frame::default();

    frame::Frame::default();

    bp.set_size(&wf, 60);
    bp.set_size(&urow, 30);
    bp.set_size(&prow, 30);
    bp.set_size(&pad, 1);
    bp.set_size(&brow, 30);
    bp.set_size(&b, 30);

    bp.end();

    frame::Frame::default();

    mp.set_size(&frame, 200);
    mp.set_size(&spacer, 10);
    mp.set_size(&bp, 300);

    mp.end();

    frame::Frame::default();

    col.end();

    grp1.end();

    let mut bf = frame::Frame::new(50, 100, 150, 30, "Benutzer:");
    let mut rf = frame::Frame::new(50, 150, 150, 30, "Rolle:");

    let mut grp2 = group::Group::default().size_of(&wizard);

    let mut inp = input::Input::default()
        .with_label("Barcode:")
        .with_size(320, 30)
        .with_pos(50, 200);
    inp.set_trigger(enums::CallbackTrigger::EnterKey);
    inp.set_callback(|i| process_barcode(i));

    let mut backb = button::Button::new(25, h - 50, 100, 30, "Abmelden");

    grp2.add(&bf);
    grp2.add(&rf);
    grp2.end();

    wizard.end();

    backb.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    login.set_callback(move |_| {
        let res = loginfn(username.value(), password.value());

        println!("{:?}", res);

        // let token = match res {
        //     Ok(t) => t,
        //     Err(e) => {
        //         println!("Error: {}", e);
        //         return;
        //     }
        // };

        // println!("token2: {}", token);

        let benutzer = ["Benutzer: ", &username.value()].concat();
        bf.set_label(&benutzer);

        rf.set_label("Rolle: - todo");

        // start looper in new thread
        std::thread::spawn(|| looper());

        let mut wiz_c = wizard.clone();
        wiz_c.next();
    });

    win.end();
    win.show();

    a.run().unwrap();
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

fn find_device() -> RawInputManager {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();

    // println!("Devices: {:?}", devices);

    let keyboards = Arc::new(devices.keyboards);

    // println!("Keyboards: {:?}", keyboards);

    // filter keyboard which contains 'VID_0483&PID_5750', send alert if not found
    let keyboard = keyboards
        .iter()
        .find(|k| k.name.contains("VID_0483&PID_5750"))
        .unwrap_or_else(|| {
            // dialog::alert_default("Barcodescanner nicht gefunden, bitte anstecken und einschalte und Programm erneut starten");
            println!("Keyboard not found");
            dialog::alert_default("Barcodescanner nicht gefunden. Bitte anschliessen, einschalten und Programm neu starten");
            std::process::exit(1);
        });
    // println!("Keyboard: {:?}", keyboard);
    // gf.set_label(&keyboard.name);

    manager.filter_devices(vec![keyboard.name.clone()]);
    return manager;
}

fn looper() {
    let mut manager = find_device();

    let mut switch_back_hwd = unsafe { winapi::um::winuser::GetForegroundWindow() };

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
