use crate::{logo_and_version::logo_and_version, GJWT, OFFLINE, USER_ID};
use fltk::menu::Choice;
use fltk::{
    button, dialog, enums,
    examples::wizard,
    frame, group, input,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt},
};
use fun::{looper::looper, username_camelcase::username_camelcase};
use notify_rust::Notification;
use req::{
    get_lager_users::get_lager_users,
    loginfn::{loginfn, JWT},
};

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
                        let username = user.as_ref().unwrap().username.clone();
                        rolle = user.as_ref().unwrap().rolle.clone();

                        let users = get_lager_users(gjwt.clone()).unwrap();
                        println!("users: {:?}", users);

                        let lager_users = get_lager_users(gjwt)
                            .unwrap()
                            .into_iter()
                            .filter(|u| u.username != username)
                            .collect::<Vec<_>>();
                        for user in lager_users.iter() {
                            lager_choice1.add_choice(&user.username);
                            lager_choice2.add_choice(&user.username);
                        }

                        // unsafe { USER_ID = user.as_ref().unwrap().id.to_string() };
                        unsafe { USER_ID = user.as_ref().unwrap().id };

                        Notification::new()
                            .summary(&format!(
                                "Barcode Scanner: {} hat sich angemeldet",
                                username
                            ))
                            .show()
                            .unwrap();

                        let inp_c = barcode_input.clone();
                        let chce_c = device_choice.clone();
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
                if e.to_string().contains("os error 10061") {
                    // dialog::alert_default("Server nicht erreichbar");
                    println!("Error e: {}", e);

                    // ask is user lager or production
                    // let mut dlg = Choice::new(0, 0, 400, 300, "Server nicht erreichbar");

                    let choice = dialog::choice2_default(
                        "Server nicht erreichbar, arbeitest du in der Produktion oder im Lager?",
                        "Abbrechen",
                        "Produktion",
                        "Lager",
                    );
                    println!("{}", choice.unwrap());

                    if choice.unwrap() == 1 {
                        rolle = "Produktion".to_string();
                    } else if choice.unwrap() == 2 {
                        rolle = "Lager".to_string();
                    } else {
                        return;
                    }
                } else {
                    println!("Error e: {}", e);
                    dialog::alert_default(&e.to_string());
                }
            }
        }

        benutzername_output.set_value(&username);
        rolle_output.set_value(&rolle);

        // if GJWT then:
        if unsafe {GJWT.is_empty() } {
            if rolle == "Lager" {
                // ask for mitarbeiter1 name and mitarbeiter2 name
                let mitarbeiter1 = dialog::input_default("Lager Mitarbeiter 1", "");
                let mitarbeiter2 = dialog::input_default("Lager Mitarbeiter 2", "");

                // check the Option<String> mitarbeiter1 and mitarbeiter2
                if mitarbeiter1.is_none() || mitarbeiter2.is_none() {
                    dialog::alert_default("Mitarbeiter 1 und Mitarbeiter 2 sind Pflichtfelder");
                    return;
                }
                mitarbeiter1_output.set_value(&mitarbeiter1.unwrap());
                mitarbeiter2_output.set_value(&mitarbeiter2.unwrap());
                wizard.next();
                wizard.next();
            } else {
                mitarbeiter1_output.set_value("");
                mitarbeiter2_output.set_value("");
                mitarbeiter1_output.hide();
                mitarbeiter2_output.hide();
                wizard.next();
                wizard.next();
            }
        }

        wizard.next();
    });
}
