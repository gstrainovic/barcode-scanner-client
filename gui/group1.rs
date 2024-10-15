#![allow(unused_assignments)]

use crate::{logo_and_version::logo_and_version, GJWT, USER_ID};
use fltk::{
    button, dialog, enums, frame, group, input,
    prelude::{GroupExt, InputExt, MenuExt, WidgetExt},
};
use fun::{looper::looper, username_camelcase::username_camelcase};
use notify_rust::Notification;
use req::loginfn::User;
use req::{
    get_lager_users::get_lager_users,
    get_users::get_users,
    loginfn::{loginfn, JWT},
};
use sqlite::{get_lager_users as sq_get_lager_users, update_users};

use std::sync::atomic::{AtomicBool, Ordering};

// Statische Variable zur Verfolgung des `looper`-Threads
static LOOPER_RUNNING: AtomicBool = AtomicBool::new(false);

fn start_looper(barcode_input: fltk::input::Input, device_choice: fltk::menu::Choice, rolle: String) {
    // Überprüfen, ob der `looper`-Thread bereits läuft
    if !LOOPER_RUNNING.load(Ordering::SeqCst) {
        println!("Starte Looper");
        LOOPER_RUNNING.store(true, Ordering::SeqCst);
        let inp_c = barcode_input.clone();
        let chce_c = device_choice.clone();
        let rol_c = rolle.clone();
        let jwt_c = unsafe { GJWT.clone() };
        std::thread::spawn(move || {
            looper(inp_c, chce_c, rol_c, jwt_c);
            LOOPER_RUNNING.store(false, Ordering::SeqCst); // Setzen Sie den Status zurück, wenn der Thread beendet ist
        });
    } else {
        println!("Looper läuft bereits");
    }
}


pub fn group1(
    mut wizard: group::Wizard,
    mut lager_choice1: fltk::menu::Choice,
    mut lager_choice2: fltk::menu::Choice,
    mut mitarbeiter1_output: fltk::output::Output,
    mut mitarbeiter2_output: fltk::output::Output,
    mut benutzername_output: fltk::output::Output,
    mut rolle_output: fltk::output::Output,
    barcode_input: fltk::input::Input,
    device_choice: fltk::menu::Choice,
) -> () {
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

    let mut password_label = frame::Frame::default()
        .with_label("Passwort:")
        .with_align(enums::Align::Inside | enums::Align::Right);
    grid.insert_ext(&mut password_label, 10, 0, 1, 1);

    let mut password = input::SecretInput::default();
    grid.insert_ext(&mut password, 10, 1, 1, 1);

    let mut login_button = button::ReturnButton::default().with_label("Anmelden");
    grid.insert_ext(&mut login_button, 12, 1, 1, 1);

    grp1.end();

    login_button.set_callback(move |_| {
        let username = username_camelcase(user_input.value());
        let res = loginfn(username.clone(), password.value());
        let mut rolle = String::new();
        let mut continue_bool = false;
        match res {
            Ok(j) => {
                match j {
                    JWT {
                        user,
                        jwt,
                        error: None,
                    } => {
                        
                        unsafe { GJWT = jwt.unwrap() };
                        let gjwt = unsafe { GJWT.clone() };
                        rolle = user.as_ref().unwrap().rolle.clone();

                        let users = get_users(gjwt.clone()).unwrap();
                        if users.len() > 0 {
                            update_users(users);
                        }
                        unsafe { USER_ID = user.as_ref().unwrap().id };
                        continue_bool = true;
                    }
                    _ => {
                        // println!("Error j : {:?}", j);
                        dialog::alert_default("Benutzername oder Passwort falsch");
                        continue_bool = false;
                    }
                }
            }
            Err(_) => {
                // if e.to_string().contains("os error 10061") {
                    // println!("Error e: {}", e);
                    dialog::message_default(
                        "Server nicht erreichbar, speichere die Daten lokal, wird beim nächsten Start synchronisiert",
                    );
                    let user = sqlite::get_user(username.clone());
                    if user.is_none() {
                        dialog::alert_default("Benutzer nicht in der lokalen Datenbank vorhanden");
                        continue_bool = false;
                        return;
                    }
                    let user_copy = user.unwrap();
                    rolle = user_copy.rolle.clone();
                    unsafe { USER_ID = user_copy.strapi_id };
                    continue_bool = true;
                // } else {
                    // println!("Error e: {}", e);
                    // dialog::alert_default(&e.to_string());
                    // continue_bool = false;
                    // return;
                // }
            }
        }

        if !continue_bool {
            return;
        }

        Notification::new()
        .summary(&format!(
            "Barcode Scanner: {} hat sich angemeldet",
            username
        ))
        .show()
        .unwrap();

        benutzername_output.set_value(&username);
        rolle_output.set_value(&rolle);

        let mut lager_users: Vec<User> = Vec::new();

        let offline = unsafe { GJWT == "" };

        if offline {
            // load lager users from sqlite
            let sq_lager_users = sq_get_lager_users();
            //transform sqlite users to reqwest users
            for sq_lager_user in sq_lager_users {
                let lager_user = User {
                    id: sq_lager_user.strapi_id,
                    username: sq_lager_user.username,
                    rolle: sq_lager_user.rolle,
                };
                lager_users.push(lager_user);
            }
        } else {
            lager_users = get_lager_users(unsafe { GJWT.clone() })
            .unwrap()
        }

        // remove same user from lager_users
        lager_users = lager_users
            .into_iter()
            .filter(|u| u.username != username)
            .collect::<Vec<_>>();

        for user in lager_users.iter() {
            lager_choice1.add_choice(&user.username);
            lager_choice2.add_choice(&user.username);
        }



        if offline {
            if rolle == "Lager" {
                wizard.next();
                return;
            } else {
                start_looper(barcode_input.clone(), device_choice.clone(), rolle.clone());
                mitarbeiter1_output.set_value("");
                mitarbeiter2_output.set_value("");
                mitarbeiter1_output.hide();
                mitarbeiter2_output.hide();
                wizard.next();
                wizard.next();
                return;
            }
        }

        if rolle == "Lager" {
            wizard.next();
            return;
        } else {
            start_looper(barcode_input.clone(), device_choice.clone(), rolle);
            wizard.next();
            wizard.next();
            return;
        }
    });
}