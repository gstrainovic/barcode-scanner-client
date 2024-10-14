use fltk::{
    button, frame, group,
    menu::Choice,
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetExt},
};
use req::get_lager_users::get_lager_users;
use sqlite::get_lager_users as sq_get_lager_users;
use req::loginfn::User;

use crate::{logo_and_version::logo_and_version, LAGER_USER_IDS, GJWT};

pub fn group2(
    mut wizard: group::Wizard,
    mut mitarbeiter1_output: Output,
    mut mitarbeiter2_output: Output,
    mut lager_choice1: Choice,
    mut lager_choice2: Choice,
) {
    let grp_lager = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();
    grid.set_layout(22, 9);

    let mut wer_hilft_label = frame::Frame::default().with_label("Wer hilft dir beim Verpacken?");
    grid.insert_ext(&mut wer_hilft_label, 7, 3, 3, 1);

    let mut arbeite_alleine_checkbox = button::CheckButton::default().with_label("Ich arbeite alleine");
    grid.insert_ext(&mut arbeite_alleine_checkbox, 9,4, 3, 1);

    lager_choice1.set_label("Mitarbeiter 1");
    lager_choice2.set_label("Mitarbeiter 2");

    arbeite_alleine_checkbox.set_callback({
        let mut lager_choice1_c = lager_choice1.clone();
        let mut lager_choice2_c = lager_choice2.clone();
        move |b| {
            if b.is_checked() {
                lager_choice1_c.hide();
                lager_choice2_c.hide();
            } else {
                lager_choice1_c.show();
                lager_choice2_c.show();
            }
        }
    });

    grid.insert_ext(&mut lager_choice1, 11, 3, 3, 1);
    grid.insert_ext(&mut lager_choice2, 13, 3, 3, 1);

    let mut lager_button_zurueck = button::Button::default().with_label("Zurück");
    grid.insert_ext(&mut lager_button_zurueck, 15, 3, 1, 1);

    let mut lager_button_weiter = button::ReturnButton::default().with_label("Weiter");
    grid.insert_ext(&mut lager_button_weiter, 15, 5, 1, 1);

    grp_lager.end();

    lager_button_zurueck.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    let lager_choice1_c = lager_choice1.clone();
    let lager_choice2_c = lager_choice2.clone();
    lager_button_weiter.set_callback(move |_| {
        let mut lager_user_choices: Vec<String> = Vec::new();
        match lager_choice1_c.choice() {
            Some(x) => {
                if x != "-" {
                    mitarbeiter1_output.set_value(&x);
                    mitarbeiter1_output.show();
                    lager_user_choices.push(x);
                } else {
                    mitarbeiter1_output.set_value("");
                    mitarbeiter1_output.hide();
                }
            }
            None => {
                mitarbeiter1_output.set_value("");
                mitarbeiter1_output.hide();
            }
        }
        match lager_choice2_c.choice() {
            Some(x) => {
                if x != "-" {
                    mitarbeiter2_output.set_value(&x);
                    mitarbeiter2_output.show();
                    lager_user_choices.push(x);
                } else {
                    mitarbeiter2_output.set_value("");
                    mitarbeiter2_output.hide();
                }
            }
            None => {
                mitarbeiter2_output.set_value("");
                mitarbeiter2_output.hide();
            }
        }

        if mitarbeiter1_output.value() == mitarbeiter2_output.value() && mitarbeiter1_output.value() != "" && mitarbeiter2_output.value() != "" {
            let message = "Mitarbeiter 1 und Mitarbeiter 2 dürfen nicht gleich sein!";
            // println!("{}", message);
            fltk::dialog::alert_default(message);
            return;
        }

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
            lager_users = get_lager_users(unsafe { GJWT.clone() }).unwrap();
        }

        unsafe { LAGER_USER_IDS.clear() };
        for lager_user_choice in lager_user_choices.clone() {
            for lager_user in &lager_users {
                if lager_user_choice == lager_user.username {
                    unsafe {
                        LAGER_USER_IDS.push(lager_user.id);
                    }
                }
            }
        }


        wizard.next();
    });
}
