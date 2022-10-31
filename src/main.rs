use std::sync::Arc;

use barcode_scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::menu::Choice;
use fltk::{app, button, group, window};
use fltk::{prelude::*, *};
use fltk_theme::{ThemeType, WidgetTheme};
use notify_rust::Notification;
use self_update::cargo_crate_version;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use winapi::shared::windef::HWND__;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

fn get_url() -> String {
    let f = std::fs::File::open("config.yaml").unwrap();
    let data: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
    data["url"].as_str().map(|s| s.to_string()).unwrap()
}

#[derive(Deserialize, Debug)]
struct JWT {
    jwt: Option<String>,
    error: Option<Map<String, Value>>,
    user: Option<User>,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    id: i16,
    rolle: String,
}

#[derive(Deserialize, Debug)]
struct BarcodeData {
    data: IdAtr,
}

#[derive(Deserialize, Debug)]
struct IdAtr {
    id: i16,
    attributes: Map<String, Value>,
}

fn logo_and_version() {
    let mut logoframe = frame::Frame::default().with_size(200, 100);
    let mut logosvg = image::SvgImage::load("gravurzeile-logo.svg").unwrap();
    logosvg.scale(200, 100, true, true);
    logoframe.set_image(Some(logosvg));
    logoframe.set_pos(10, 10);
    let mut version = frame::Frame::default().with_size(100, 20);
    version.set_label(&format!("Version {}", cargo_crate_version!()));
    version.set_pos(10, 110);
}

#[tokio::main]
async fn write_barcode(
    barcode: String,
    user: i16,
    jwt: &str,
) -> Result<BarcodeData, reqwest::Error> {
    let url = get_url() + "/api/barcodes";

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&json!({
          "data": {
            "barcode": barcode,
            "users_permissions_user": user
          }
        }))
        .send()
        .await?;

    let body = res.text().await?;

    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}

#[tokio::main]
async fn loginfn(user: String, pass: String) -> Result<JWT, reqwest::Error> {
    let url = get_url() + "/api/auth/local";

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(&url)
        .json(&json!({
          "identifier": user,
          "password": pass
        }))
        .send()
        .await?;

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}

fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("gstrainovic")
        .repo_name("barcode-scanner-client")
        .bin_name("barcode_scanner.exe")
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        let message = format!(
            "Aktualisiert zu {}. Bitte barcode_scanner.exe nochmals starten",
            status.version()
        );
        println!("{}", message);
        dialog::alert_default(&message);
        return Err(Box::new(self_update::errors::Error::Update(message)));
    } else {
        println!("Already up to date");
        return Ok(());
    }

    // Ok(())
}

fn get_hwnd_of_barcode_scanner() -> *mut HWND__ {
    let my_windows_hwnd = unsafe {
        winapi::um::winuser::FindWindowA(std::ptr::null(), "BarcodeScanner\0".as_ptr() as *const i8)
    };
    return my_windows_hwnd;
}


fn main() {
    hide_console_window();
    update().unwrap();

    let hwnd_of_barcode_scanner = get_hwnd_of_barcode_scanner();

    if hwnd_of_barcode_scanner != std::ptr::null_mut() {
        let message = "Barcodescanner läuft bereits!";
        println!("{}", message);
        dialog::alert_default(message);
        return;
    }

    let w = 640;
    let h = 480;

    let a = app::App::default().with_scheme(app::Scheme::Gleam);
    app::set_visible_focus(true);

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

    win.make_resizable(true);

    let my_windows_hwnd = unsafe {
        winapi::um::winuser::FindWindowA(std::ptr::null(), "BarcodeScanner\0".as_ptr() as *const i8)
    };

    // maximize window
    unsafe {
        winapi::um::winuser::ShowWindow(my_windows_hwnd, winapi::um::winuser::SW_MAXIMIZE);
    }

    let mut wizard = group::Wizard::default().with_size(w, h);

    let grp0 = group::Group::default().size_of(&wizard);
    logo_and_version();

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let mut chce = Choice::default().with_size(300, 30);
    chce.set_pos(120, 150);
    chce.set_label("Gerät auswählen");
    let keyboards = Arc::new(devices.keyboards);
    for keyboard in keyboards.iter() {
        chce.add_choice(&keyboard.name);
    }

    let mut btn = button::ReturnButton::default().with_label("Weiter");
    // btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn.set_size(410, 30);
    btn.set_pos(10, 200);
    btn.hide();

    grp0.end();

    let grp1 = group::Group::default().size_of(&wizard);

    let col = group::Flex::default_fill().column();
    frame::Frame::default();

    let mut mp = group::Flex::default().row();

    frame::Frame::default();

    // Takes a path
    let mut frame = frame::Frame::default();
    let mut myimage = image::SvgImage::load("gravurzeile-logo.svg").unwrap();
    myimage.scale(200, 200, true, true);
    frame.set_image(Some(myimage));

    // add icon
    let image = image::PngImage::load("gravurzeile-favicon-32x32.png").unwrap();
    win.set_icon(Some(image));

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

    let mut grp2 = group::Group::default().size_of(&wizard);

    let mut bf = output::Output::new(150, 150, 150, 30, "Benutername");
    let mut backb = button::Button::new(320, 150, 150, 30, "Abmelden");
    let mut rf = output::Output::new(150, 200, 150, 30, "Rolle");

    let inp = input::Input::default()
        .with_label("Barcode:")
        .with_size(320, 30)
        .with_pos(150, 250);
    // inp.set_trigger(enums::CallbackTrigger::EnterKey);

    let mut sendenb = button::ReturnButton::new(150, 320, 320, 30, "Senden");

    grp2.add(&bf);
    grp2.add(&rf);
    grp2.end();

    wizard.end();

    backb.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    btn.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.next()
    });

    chce.set_callback({
        // let mut btn_c = btn.clone();
        move |_| {
            btn.show();
        }
    });

    let mut guser = None;
    let mut gjwt = String::new();

    login.set_callback(move |_| {
        // transform username to first letter uppercase and rest lowercase
        let mut uname = username.value();
        uname = uname.to_lowercase();
        let mut uname = uname.chars();
        let first = uname.next().unwrap().to_uppercase();
        let rest: String = uname.collect();
        let uname = format!("{}{}", first, rest);
        println!("Username: {}", uname);
        let res = loginfn(uname, password.value());
        println!("{:?}", res);

        match res {
            Ok(j) => {
                match j {
                    JWT {
                        user,
                        jwt,
                        error: None,
                    } => {
                        guser = user;
                        gjwt = jwt.unwrap();
                        wizard.next();

                        println!("User: {:?}", guser);

                        println!("JWT: {:?}", gjwt);

                        let username = guser.as_ref().unwrap().username.clone();

                        bf.set_value(&username);
                        rf.set_value(guser.as_ref().unwrap().rolle.as_str());

                        let user_id = guser.as_ref().unwrap().id;

                        let jwt = gjwt.clone();

                        let mut inp_c = inp.clone();

                        // send notification
                        let mut notif = Notification::new();
                        notif.summary("Anmeldung");
                        notif.body(&format!("{} hat sich angemeldet", username));
                        notif.show().unwrap();

                        sendenb.set_callback(move |_| {
                            process_barcode(&mut inp_c, user_id, &jwt);
                        });

                        // start looper in new thread
                        let inp_c = inp.clone();
                        let chce_c = chce.clone();
                        std::thread::spawn(|| looper(inp_c, chce_c));
                    }
                    JWT {
                        user: None,
                        jwt: None,
                        error: Some(err),
                    } => {
                        println!("Error err: {:?}", err);
                        match err.get_key_value("message") {
                            Some((k, v)) => {
                                let value_s = v.as_str().unwrap();
                                match value_s {
                                    "Invalid identifier or password" => {
                                        println!("{}", value_s);
                                        dialog::alert_default("Benutzername oder Passwort falsch");
                                    }
                                    "password is a required field" => {
                                        println!("{}", value_s);
                                        dialog::alert_default("Passwort ist ein Pflichtfeld");
                                    }
                                    "username is a required field" => {
                                        println!("{}", value_s);
                                        dialog::alert_default("Benutzer ist ein Pflichtfeld");
                                    }
                                    "2 errors occurred" => {
                                        println!("{}", value_s);
                                        dialog::alert_default(
                                            "Benutzername und Passwort sind Pflichtfelder",
                                        );
                                    }
                                    _ => {
                                        println!("Error2: {:?}", value_s);
                                        dialog::alert_default(value_s);
                                    }
                                }
                                println!("Error: {} {}", k, v);
                            }
                            None => {
                                println!("Error: {:?}", err);
                            }
                        }
                    }
                    _ => {
                        println!("Error j : {:?}", j);
                    }
                }
            }
            Err(e) => {
                println!("Error e: {}", e);
                dialog::alert_default(&e.to_string());
            }
        }
    });

    win.end();
    win.show();
    // win.maximize();
    win.activate();

    a.run().unwrap();
}

fn create_button(caption: &str) -> button::ReturnButton {
    let mut btn = button::ReturnButton::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn.set_size(500, 30);
    btn
}

fn process_barcode(i: &mut input::Input, user: i16, jwt: &str) {
    i.activate();

    let barcode = i.value();

    if barcode.is_empty() {
        dialog::alert_default("Barcode ist ein Pflichtfeld");
        return;
    } else {
        let resp = write_barcode(i.value(), user, jwt);

        match resp {
            Ok(_) => {
                println!("Barcode gesendet: {}", barcode);
                let mut notif = Notification::new();
                notif.summary("Barcode gesendet:");
                notif.body(&barcode);
                notif.show().unwrap();
                i.set_value("");
            }
            Err(e) => {
                println!("Error: {}", e);
                dialog::alert_default(e.to_string().as_str());
            }
        }
    }
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

fn looper(mut inp: input::Input, chce: Choice) {
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
