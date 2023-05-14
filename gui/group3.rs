use fltk::{group, button, output, input, prelude::{WidgetExt, GroupExt}};

use crate::logo_and_version;

pub fn group3(
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
