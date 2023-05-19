use std::sync::Arc;

use fltk::{menu::Choice, prelude::{WidgetExt, MenuExt}};
use scanner::{RawInputManager, DeviceType};

pub fn device_choice() -> Choice {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();
    let mut chce = Choice::default(); //.with_size(300, 30);
    chce.set_label("Gerät auswählen");
    let keyboards = Arc::new(devices.keyboards);
    for keyboard in keyboards.iter() {
        chce.add_choice(&keyboard.name);
    }
    chce
}