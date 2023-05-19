use fltk::{group, button, input, prelude::{WidgetExt, GroupExt, InputExt, MenuExt}, frame, enums, dialog};
use fun::{looper::looper, username_camelcase::username_camelcase};
use notify_rust::Notification;
use req::{loginfn::{JWT, loginfn}, get_lager_users::get_lager_users};
use crate::{logo_and_version::logo_and_version, GJWT, USER_ID};

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
        let res = loginfn(username, password.value());
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
                        let rolle = user.as_ref().unwrap().rolle.clone();
                        let lager_users = get_lager_users(gjwt).unwrap()
                            .into_iter()
                            .filter(|u| u.username != username)
                            .collect::<Vec<_>>();
                        for user in lager_users.iter() {
                            lager_choice1.add_choice(&user.username);
                            lager_choice2.add_choice(&user.username);
                        }
                        if rolle == "Lager" {
                            wizard.next();
                        } else {
                            mitarbeiter1_output.set_value("");
                            mitarbeiter2_output.set_value("");
                            mitarbeiter1_output.hide();
                            mitarbeiter2_output.hide();
                            wizard.next();
                            wizard.next();
                        }

                        benutzername_output.set_value(&username);
                        rolle_output.set_value(user.as_ref().unwrap().rolle.as_str());

                        unsafe { USER_ID = user.as_ref().unwrap().id.to_string() };

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
                println!("Error e: {}", e);
                dialog::alert_default(&e.to_string());
            }
        }
    });
}
