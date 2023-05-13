use std::sync::Arc;

use fltk::{group, button, prelude::{WidgetExt, GroupExt, MenuExt}, menu::Choice};
use scanner::{RawInputManager, DeviceType};
use crate::{logo_and_version::logo_and_version};

fn device_choice() -> Choice {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let mut chce = Choice::default(); //.with_size(300, 30);
                                      // chce.set_pos(120, 150);
    chce.set_label("Gerät auswählen");
    let keyboards = Arc::new(devices.keyboards);
    for keyboard in keyboards.iter() {
        chce.add_choice(&keyboard.name);
    }
    chce
}

pub fn group0(wizard: group::Wizard) -> Choice {
    let grp0 = group::Group::default().size_of(&wizard);

    let mut next_button = button::ReturnButton::default().with_label("Weiter"); //.hide();
    next_button.hide();

    let mut chce = device_choice();

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

