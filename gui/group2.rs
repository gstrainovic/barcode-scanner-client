
use fltk::{group, prelude::{WidgetExt, GroupExt, MenuExt, InputExt}, button, frame, menu::Choice, output::Output};

use crate::logo_and_version::logo_and_version;

pub fn group2(mut wizard: group::Wizard, mut m1: Output, mut m2: Output) -> (Choice, Choice) {
    let grp_lager = group::Group::default().size_of(&wizard);
    let mut grid = logo_and_version();
    let mut lager_frame = frame::Frame::default()
        .with_label("Wer hilft dir beim Verpacken?");
    grid.insert_ext(&mut lager_frame, 7, 1, 1, 1);

    // two choice to select 1-2 colleagues
    let mut lager_choice1 =  Choice::default();
    lager_choice1.set_label("Mitarbeiter 1");

    let mut lager_choice2 =  Choice::default();
    lager_choice2.set_label("Mitarbeiter 2");

    grid.insert_ext(&mut lager_choice1, 9, 1, 1, 1);
    grid.insert_ext(&mut lager_choice2, 11, 1, 1, 1);

    //lager_button_zurueck
    let mut lager_button_zurueck = button::Button::default().with_label("Zurück");
    grid.insert_ext(&mut lager_button_zurueck, 13, 1, 1, 1);

    //lager_button_weiter
    let mut lager_button_weiter = button::ReturnButton::default().with_label("Weiter");
    grid.insert_ext(&mut lager_button_weiter, 15, 1, 1, 1);

    grp_lager.end();

    //lager_button_zurueck funktion
    lager_button_zurueck.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    let lager_choice1_c = lager_choice1.clone();
    let lager_choice2_c = lager_choice2.clone();
    lager_button_weiter.set_callback(move |_| {
        match lager_choice1_c.choice() {
            Some(x) => {
                m1.set_value(&x);
                m1.show();
            }
            None => (),
        }
        match lager_choice2_c.choice() {
            Some(x) => {
                m2.set_value(&x);
                m2.show();
            }
            None => (),
        }

        if m1.value() == m2.value() && m1.value() != "" && m2.value() != "" {
            let message = "Mitarbeiter 1 und Mitarbeiter 2 dürfen nicht gleich sein!";
            println!("{}", message);
            fltk::dialog::alert_default(message);
            return;
        }

        wizard.next();
    });



    return (lager_choice1, lager_choice2);

}

