use fltk::{group, button, output, input, prelude::{WidgetExt, GroupExt}};

use crate::logo_and_version;

pub fn group3(
    wizard: group::Wizard,
    mut m1: output::Output,
    mut m2: output::Output,
    // ) -> (group::Group, button::Button, output::Output, output::Output, input::Input, button::ReturnButton) {
) -> (
    output::Output,
    output::Output,
    input::Input,
    button::ReturnButton,
) {
    let grp2 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();

    let mut bf = output::Output::default().with_label("Benutzername");
    grid.insert_ext(&mut bf, 7, 1, 1, 1);

    let mut rf = output::Output::default().with_label("Rolle");
    grid.insert_ext(&mut rf, 8, 1, 1, 1);

    // let mut m1 = output::Output::default().with_label("Mitarbeiter 1");
    grid.insert_ext(&mut m1, 9, 1, 1, 1);

    // let mut m2 = output::Output::default().with_label("Mitarbeiter 2");
    grid.insert_ext(&mut m2, 10, 1, 1, 1);

    let mut backb = button::Button::default().with_label("Abmelden");
    grid.insert_ext(&mut backb, 12, 1, 1, 1);

    let mut inp = input::Input::default().with_label("Barcode:");
    grid.insert_ext(&mut inp, 14, 1, 1, 1);

    let mut sendenb = button::ReturnButton::default().with_label("Senden");
    grid.insert_ext(&mut sendenb, 16, 1, 1, 1);

    grp2.end();
    
    backb.set_callback(move |_| { 
        wizard.clone().prev();
        wizard.clone().prev();
        }
    );

    (bf, rf, inp, sendenb)
}
