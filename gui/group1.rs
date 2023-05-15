use fltk::{group, button, input, prelude::{WidgetExt, GroupExt, InputExt, MenuExt}, frame, enums, dialog};
use fun::looper::looper;
use notify_rust::Notification;
use req::{loginfn::{JWT, loginfn}, get_lager_users::get_lager_users};
use crate::{logo_and_version::logo_and_version, GJWT};

pub fn group1(
    mut wizard: group::Wizard,
    mut lager_choice1: fltk::menu::Choice,
    mut lager_choice2: fltk::menu::Choice,
    mut m1: fltk::output::Output,
    mut m2: fltk::output::Output,
    mut bf: fltk::output::Output,
    mut rf: fltk::output::Output,
    mut user_id: fltk::output::Output,
    inp: fltk::input::Input,
    chce: fltk::menu::Choice,
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

    let mut pframe = frame::Frame::default()
        .with_label("Passwort:")
        .with_align(enums::Align::Inside | enums::Align::Right);
    grid.insert_ext(&mut pframe, 10, 0, 1, 1);

    let mut password = input::SecretInput::default();
    grid.insert_ext(&mut password, 10, 1, 1, 1);

    let mut login_button = button::ReturnButton::default().with_label("Anmelden");
    grid.insert_ext(&mut login_button, 12, 1, 1, 1);

    grp1.end();

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
                        let guser = user;
                        unsafe { GJWT = jwt.unwrap() };

                        let gjwt = unsafe { GJWT.clone() };
                        
                        println!("User: {:?}", guser);
                        println!("JWT: {:?}", gjwt);

                        let username = guser.as_ref().unwrap().username.clone();
                        let rolle = guser.as_ref().unwrap().rolle.clone();

                        println!("Username: {}", username);
                        println!("Rolle: {}", rolle);
                        let lager_users = get_lager_users(gjwt).unwrap()
                            .into_iter()
                            .filter(|u| u.username != username)
                            .collect::<Vec<_>>();
                        println!("Lager users: {:?}", lager_users);
                        for user in lager_users.iter() {
                            lager_choice1.add_choice(&user.username);
                            lager_choice2.add_choice(&user.username);
                        }

                        // add empty choice to lager_choice1 and lager_choice2


                        if rolle == "Lager" {
                            wizard.next();
                        } else {
                            m1.set_value("");
                            m2.set_value("");
                            m1.hide();
                            m2.hide();
                            wizard.next();
                            wizard.next();
                        }

                        bf.set_value(&username);
                        rf.set_value(guser.as_ref().unwrap().rolle.as_str());

                        user_id.set_value(&guser.as_ref().unwrap().id.to_string());
                        // let jwt = gjwt.clone();

                        Notification::new()
                            .summary(&format!(
                                "Barcode Scanner: {} hat sich angemeldet",
                                username
                            ))
                            .show()
                            .unwrap();

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
}
