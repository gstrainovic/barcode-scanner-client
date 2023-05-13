use config::STRAPI_URL;

use fun::{
    process_barcode::process_barcode,
    looper::looper,
};
use req::loginfn::JWT;
use ::scanner::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use ::req::{get_ausnahmen::get_ausnahmen, loginfn::loginfn};
use fltk::app::screen_size;
use fltk::frame::Frame;
use fltk::menu::Choice;
use fltk::{app, button, group, window};
use fltk::{prelude::*, *};
use fltk_grid::Grid;
use fltk_theme::{ThemeType, WidgetTheme};
use notify_rust::Notification;

use serde::Deserialize;
use serde_json::{json, Map, Value};
use winapi::shared::windef::HWND__;
use crate::logo_and_version::logo_and_version;
use crate::group1::group1;
use crate::group0::group0;
use crate::group2::group2;
use crate::hide_console_windows::hide_console_window;
use crate::get_hwnd_barcode_scanner::get_hwnd_barcode_scanner;
use crate::win::win;
use ::fun::update::update;
use ::fun::send_barcode::send_barcode;

mod logo_and_version;
mod group1;
mod group0;
mod group2;
mod hide_console_windows;
mod get_hwnd_barcode_scanner;
mod win;
// mod fun::update;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

fn main() {
    println!("STRAPI_URL: {}", STRAPI_URL);
    hide_console_window();
    update().unwrap();

    let hwnd_of_barcode_scanner = get_hwnd_barcode_scanner();

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
    win.activate();

    unsafe {
        winapi::um::winuser::ShowWindow(hwnd_of_barcode_scanner, winapi::um::winuser::SW_MAXIMIZE);
        winapi::um::winuser::SetForegroundWindow(hwnd_of_barcode_scanner);
        winapi::um::winuser::SetActiveWindow(hwnd_of_barcode_scanner);
    }

    a.run().unwrap();
}