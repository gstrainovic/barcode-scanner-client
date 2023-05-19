use fltk::{group, button, prelude::{WidgetExt, GroupExt}, menu::{Choice, self}};
use crate::{logo_and_version::logo_and_version};

pub fn group0(wizard: group::Wizard, mut chce: menu::Choice) -> Choice {

    let grp0 = group::Group::default().size_of(&wizard);

    let mut next_button = button::ReturnButton::default().with_label("Weiter"); 
    next_button.hide();

    let mut grid = logo_and_version();
    grid.insert_ext(&mut chce, 7, 1, 1, 1);
    grid.insert_ext(&mut next_button, 9, 1, 1, 1);

    next_button.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.next()
    });

    chce.set_callback({
        move |_| {
            next_button.show();
        }
    });

    grp0.end();
    chce
}

