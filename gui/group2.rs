use fltk::{
    button, frame, group,
    menu::Choice,
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetExt}, enums,
};
use req::{get_lager_users::get_lager_users, loginfn::User};

use crate::{logo_and_version::logo_and_version, LAGER_USER_IDS, GJWT};

pub fn group2(
    mut wizard: group::Wizard,
    mut m1: Output,
    mut m2: Output,
    mut lager_choice1: Choice,
    mut lager_choice2: Choice,
) {
    let grp_lager = group::Group::default().size_of(&wizard);
    let mut grid = logo_and_version();
    // grid.debug(true);
    grid.set_layout(22, 9);

    let mut lager_frame = frame::Frame::default().with_label("Wer hilft dir beim Verpacken?");
    grid.insert_ext(&mut lager_frame, 7, 3, 3, 1);

    // check box Ich arbeite alleine
    let mut lager_check = button::CheckButton::default().with_label("Ich arbeite alleine");
    grid.insert_ext(&mut lager_check, 9,4, 3, 1);
    // center the check box
    // lager_check.set_align(enums::Align::Center);

    // two choice to select 1-2 colleagues
    lager_choice1.set_label("Mitarbeiter 1");
    lager_choice2.set_label("Mitarbeiter 2");

    // if check box is checked, hide the two choices
    lager_check.set_callback({
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

    //lager_button_zurueck
    let mut lager_button_zurueck = button::Button::default().with_label("Zurück");
    grid.insert_ext(&mut lager_button_zurueck, 15, 3, 1, 1);

    //lager_button_weiter
    let mut lager_button_weiter = button::ReturnButton::default().with_label("Weiter");
    grid.insert_ext(&mut lager_button_weiter, 15, 5, 1, 1);

    grp_lager.end();

    //lager_button_zurueck funktion
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
                    m1.set_value(&x);
                    m1.show();
                    lager_user_choices.push(x);
                } else {
                    m1.set_value("");
                    m1.hide();
                }
            }
            None => {
                m1.set_value("");
                m1.hide();
            }
        }
        match lager_choice2_c.choice() {
            Some(x) => {
                if x != "-" {
                    m2.set_value(&x);
                    m2.show();
                    lager_user_choices.push(x);
                } else {
                    m2.set_value("");
                    m2.hide();
                }
            }
            None => {
                m2.set_value("");
                m2.hide();
            }
        }

        if m1.value() == m2.value() && m1.value() != "" && m2.value() != "" {
            let message = "Mitarbeiter 1 und Mitarbeiter 2 dürfen nicht gleich sein!";
            println!("{}", message);
            fltk::dialog::alert_default(message);
            return;
        }
        println!("Lager user choices: {:?}", lager_user_choices);

        let lager_users = get_lager_users(unsafe { GJWT.clone() }).unwrap();
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
        unsafe {
            println!("Lager user ids: {:?}", LAGER_USER_IDS);
        }

        wizard.next();
    });
}
