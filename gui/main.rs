// rust-analyzer.diagnostics.disabled

const STRAPI_URL: &str = "http://146.190.19.207:1337";

use std::sync::Arc;

// use scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use ::scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::app::screen_size;
use fltk::frame::Frame;
use fltk::menu::Choice;
use fltk::{app, button, group, window};
use fltk::{prelude::*, *};
use fltk_grid::Grid;
use fltk_theme::{ThemeType, WidgetTheme};
use notify_rust::Notification;
use self_update::cargo_crate_version;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use winapi::shared::windef::HWND__;
// use Deserialize


type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

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

// fn logo_and_version() -> Grid {
//     fn logo() -> Frame {
//         let mut logo = image::SvgImage::load("gui/gravurzeile-logo.svg").unwrap();
//         let mut logoframe = frame::Frame::default(); //.with_size(200, 100);
//         logo.scale(200, 100, true, true);
//         logoframe.set_image(Some(logo));
//         logoframe
//     }

//     fn slogan() -> Frame {
//         return frame::Frame::default().with_label("Einfach persönlich schenken");
//     }

//     fn version() -> Frame {
//         return frame::Frame::default().with_label(&format!("Version {}", cargo_crate_version!()));
//     }

//     let mut grid = Grid::default_fill();
//     grid.set_layout(24, 3);
//     // widget, row, col, row_span, col_span
//     grid.insert_ext(&mut logo(), 0, 0, 3, 3);
//     grid.insert_ext(&mut slogan(), 3, 0, 3, 1);
//     grid.insert_ext(&mut version(), 5, 0, 3, 1);
//     grid
// }

// // function to check, do the barcode allready exists in the database?
// #[tokio::main]
// async fn barcode_exists(
//     barcode: String,
//     user: i16,
//     jwt: &str,
// ) -> Result<BarcodeData, reqwest::Error> {

//   let url = format!("{}api/barcodes/{}", STRAPI_URL, barcode);

// }

#[derive(Deserialize, Debug)]
struct Ausnahmen {
    Barcode: String,
    Bedeutung: String,
}

#[derive(Deserialize, Debug)]
struct AusnahmenData {
    data: Vec<IdAtrAusnahmen>,
}

#[derive(Deserialize, Debug)]
struct IdAtrAusnahmen {
    id: i16,
    attributes: Ausnahmen,
}

// get all exceptions from the database
#[tokio::main]
async fn get_ausnahmen(jwt: &str) -> Result<AusnahmenData, reqwest::Error> {
    let url = format!("{}/api/ausnahmen", STRAPI_URL);
    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<AusnahmenData>()
        .await?;
    println!("{:?}", res);
    Ok(res)
}

#[tokio::main]
async fn write_barcode(
    barcode: String,
    user: i16,
    jwt: &str,
) -> Result<BarcodeData, reqwest::Error> {
    let url = format!("{}{}", STRAPI_URL, "/api/barcodes");

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
    let url = format!("{}{}", STRAPI_URL, "/api/auth/local");

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

fn device_choice() -> Choice {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let mut chce = Choice::default(); //.with_size(300, 30);
                                      // chce.set_pos(120, 150);
    chce.set_label("Gerät auswählen");
    let keyboards = Arc::new(devices.keyboards);
    for keyboard in keyboards.iter() {
        chce.add_choice(&keyboard.name);
    }
    chce
}

fn win() -> window::Window {
    let w = screen_size().0 as i32;
    let h = screen_size().1 as i32;

    let mut win = window::Window::default().with_size(w, h);
    win.set_label("BarcodeScanner");
    win.set_callback(|w| {
        let choice = dialog::choice2_default("Barcodescanner beenden?", "Nein", "Ja", "Abbruch");
        println!("{:?}", choice);
        if choice == Some(1) {
            let mut notif = Notification::new();
            notif.summary("Barcode Scanner: Barcodescanner beendet");
            notif.show().unwrap();
            w.hide();
        }
    });

    win.make_resizable(true);

    // add icon
    let image = image::PngImage::load("gui/gravurzeile-favicon-32x32.png").unwrap();
    win.set_icon(Some(image));
    win
}

fn group0(wizard: group::Wizard) -> Choice {
    // group0 start
    let grp0 = group::Group::default().size_of(&wizard);

    let mut next_button = button::ReturnButton::default().with_label("Weiter"); //.hide();
    next_button.hide();

    let mut chce = device_choice();

    let mut grid = logo_and_version();
    grid.insert_ext(&mut chce, 7, 1, 1, 1);
    grid.insert_ext(&mut next_button, 9, 1, 1, 1);

    next_button.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.next()
    });

    chce.set_callback({
        // let mut btn_c = btn.clone();
        move |_| {
            next_button.show();
        }
    });

    grp0.end();
    chce
    // group0 end
}

fn group1(wizard: group::Wizard) -> (button::ReturnButton, input::Input, input::SecretInput) {
    // group1 start
    let grp1 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();

    let mut please_login_frame = frame::Frame::default().with_label("Bitte anmelden");
    grid.insert_ext(&mut please_login_frame, 7, 1, 1, 1);

    let mut user_frame = frame::Frame::default()
        .with_label("Benutzername:")
        .with_align(enums::Align::Inside | enums::Align::Right);
    grid.insert_ext(&mut user_frame, 9, 0, 1, 1);

    let mut user_input = input::Input::default();
    grid.insert_ext(&mut user_input, 9, 1, 1, 1);

    let mut pframe = frame::Frame::default()
        .with_label("Passwort:")
        .with_align(enums::Align::Inside | enums::Align::Right);
    grid.insert_ext(&mut pframe, 10, 0, 1, 1);

    let mut password = input::SecretInput::default();
    grid.insert_ext(&mut password, 10, 1, 1, 1);

    // let mut login_button = create_button("Anmelden");
    let mut login_button = button::ReturnButton::default().with_label("Anmelden");
    grid.insert_ext(&mut login_button, 12, 1, 1, 1);

    grp1.end();

    (login_button, user_input, password)
}

fn group2(
    wizard: group::Wizard,
    // ) -> (group::Group, button::Button, output::Output, output::Output, input::Input, button::ReturnButton) {
) -> (
    button::Button,
    output::Output,
    output::Output,
    input::Input,
    button::ReturnButton,
) {
    let grp2 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();

    let mut bf = output::Output::default().with_label("Benutername");
    grid.insert_ext(&mut bf, 7, 1, 1, 1);

    let mut rf = output::Output::default().with_label("Rolle");
    grid.insert_ext(&mut rf, 8, 1, 1, 1);

    let mut backb = button::Button::default().with_label("Abmelden");
    grid.insert_ext(&mut backb, 10, 1, 1, 1);

    let mut inp = input::Input::default().with_label("Barcode:");
    grid.insert_ext(&mut inp, 12, 1, 1, 1);

    let mut sendenb = button::ReturnButton::default().with_label("Senden");
    grid.insert_ext(&mut sendenb, 14, 1, 1, 2);

    grp2.end();

    (backb, bf, rf, inp, sendenb)
}

fn main() {
    println!("STRAPI_URL: {}", STRAPI_URL);
    hide_console_window();
    update().unwrap();



    let hwnd_of_barcode_scanner = get_hwnd_of_barcode_scanner();

    if hwnd_of_barcode_scanner != std::ptr::null_mut() {
        let message = "Barcodescanner läuft bereits!";
        println!("{}", message);
        dialog::alert_default(message);
        return;
    }

    let a = app::App::default().with_scheme(app::Scheme::Gleam);
    app::set_visible_focus(true);

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let mut win = win();

    let mut wizard = group::Wizard::default().with_size(win.width(), win.height());

    let chce = group0(wizard.clone());

    let (mut login_button, user_input, password) = group1(wizard.clone());

    let (mut backb, mut bf, mut rf, inp, mut sendenb) = group2(wizard.clone());

    wizard.end();

    backb.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    let mut guser = None;
    let mut gjwt = String::new();

    login_button.set_callback(move |_| {
        // transform username to first letter uppercase and rest lowercase
        let mut uname = user_input.value();
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

                        Notification::new()
                            .summary(&format!(
                                "Barcode Scanner: {} hat sich angemeldet",
                                username
                            ))
                            .show()
                            .unwrap();

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

    unsafe {
        winapi::um::winuser::ShowWindow(hwnd_of_barcode_scanner, winapi::um::winuser::SW_MAXIMIZE);
        winapi::um::winuser::SetForegroundWindow(hwnd_of_barcode_scanner);
        winapi::um::winuser::SetActiveWindow(hwnd_of_barcode_scanner);
        // let _ = inp.take_focus();
    }

    a.run().unwrap();
}

pub fn send_barcode(barcode: String, user: i16, jwt: &str) {
    let barcode_c = barcode.clone();
    match write_barcode(barcode, user, jwt) {
        Ok(_) => {
            Notification::new()
                .summary(&format!("Barcode Scanner: {} gesendet", barcode_c))
                .show()
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            dialog::alert_default(e.to_string().as_str());
        }
    }
}

// fn send_barcode(barcode: String, user: i16, jwt: &str) {
//     let barcode_c = barcode.clone();
//     match write_barcode(barcode, user, jwt) {
//         Ok(_) => {
//             Notification::new()
//                 .summary(&format!("Barcode Scanner: {} gesendet", barcode_c))
//                 .show()
//                 .unwrap();
//         }
//         Err(e) => {
//             println!("Error: {}", e);
//             dialog::alert_default(e.to_string().as_str());
//         }
//     }
// }

// global array for barcode history
static mut BARCODES: Vec<String> = Vec::new();

fn process_barcode(i: &mut input::Input, user: i16, jwt: &str) {
    i.activate();
    let barcode = i.value();
    let barcode_c = barcode.clone();
    i.set_value("");

    let barcode_lower = barcode.to_lowercase();

        // print the ausnahmen
    let ausnahmen = get_ausnahmen(&jwt);
    println!("Ausnahmen: {:?}", ausnahmen);

    // get barcodes from ausnahmen
    // Ausnahmen: Ok(AusnahmenData { data: [IdAtrAusnahmen { id: 1, attributes: Ausnahmen { Barcode: "0101080", Bedeutung: "Kosmische Strahlung" } }, IdAtrAusnahmen { id: 2, attributes: Ausnahmen {
    // Barcode: "0101090", Bedeutung: "Vulkanausbruch" } }] })
    let mut barcode_ausnahmen = Vec::new();
    for ausnahme in ausnahmen.unwrap().data {
        barcode_ausnahmen.push(ausnahme.attributes.Barcode);
    }

    // print the barcodes
    println!("Barcodes: {:?}", barcode_ausnahmen);

    // if barcode ends with a string from barcode_ausnahmen, then send it directly to server
    for barcode_ausnahme in barcode_ausnahmen {
        if barcode_lower.ends_with(barcode_ausnahme.to_lowercase().as_str()) {
            send_barcode(barcode_c, user, jwt);
            return;
        }
    }


    // ups express like
    // 42096242 // len 8
    // but allow
    if barcode_lower.len() < 9
    {
        Notification::new()
            .summary(&format!(
                "Barcode Scanner: {} ist zu kurz, nicht gesendet",
                barcode_c
            ))
            .show()
            .unwrap();
        return;
    }

    let f = barcode_lower.chars().nth(0).unwrap();
    let s = barcode_lower.chars().nth(1).unwrap();

    // DHL Leitcode like
    // ¨C140327619348`99000900190051
    // ¨C140327628203`99000900033018
    // 0327642113+99..

    println!("barcode: {} len: {}", barcode_lower, barcode_lower.len());
    println!("barcode contains: {}", barcode_lower.contains('+'));

    if barcode_lower.len() > 13 {
        let apostrophe = barcode_lower.chars().nth(14).unwrap();
        if (f == '¨' && s == 'c' && apostrophe == '`') || barcode_lower.contains('+') {
            Notification::new()
                .summary(&format!(
                    "Barcode Scanner: {} als DHL Leitcode erkannt, nicht gesendet",
                    barcode_c
                ))
                .show()
                .unwrap();
            return;
        }
    }

    // duplicate check
    unsafe {
        if !BARCODES.contains(&barcode_lower) {
            BARCODES.push(barcode_lower.clone());
            send_barcode(barcode_c, user, jwt)
        } else {
            Notification::new()
                .summary(&format!(
                    "Barcode Scanner: {} wurde bereits gesendet",
                    barcode_c
                ))
                .show()
                .unwrap();
            return;
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
