use fltk::{group, button, input, prelude::{WidgetExt, GroupExt}, frame, enums};
use crate::logo_and_version::logo_and_version;

pub fn group1(wizard: group::Wizard) -> (button::ReturnButton, input::Input, input::SecretInput) {
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

    (login_button, user_input, password)
}
