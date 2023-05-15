use fltk::{group, button, output, input, prelude::{WidgetExt, GroupExt, InputExt, MenuExt}, menu::Choice};
use fun::process_barcode::process_barcode;

use crate::{logo_and_version, LAGER_USER_IDS, GJWT};

pub fn group3(
    wizard: group::Wizard,
    mut m1: output::Output,
    mut m2: output::Output,
    mut user_id: output::Output,
    mut rf: output::Output,
    mut bf: output::Output,
    mut inp: input::Input,
) -> (
) {
    let grp2 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();


    grid.insert_ext(&mut bf, 7, 1, 1, 1);
    grid.insert_ext(&mut rf, 8, 1, 1, 1);
    grid.insert_ext(&mut m1, 10, 1, 1, 1);
    grid.insert_ext(&mut m2, 11, 1, 1, 1);

    let mut backb = button::Button::default().with_label("Abmelden");
    grid.insert_ext(&mut backb, 13, 1, 1, 1);


    grid.insert_ext(&mut inp, 15, 1, 1, 1);

    let mut sendenb = button::ReturnButton::default().with_label("Senden");
    grid.insert_ext(&mut sendenb, 17, 1, 1, 1);

    grp2.end();
    
    backb.set_callback(move |_| { 
        wizard.clone().prev();
        wizard.clone().prev();
        }
    );

    sendenb.set_callback(move |_| {
        unsafe {
            println!("Lager user ids as choose: {:?}", LAGER_USER_IDS);
        }
        println!("User id: {}", user_id.value());

        unsafe {
            process_barcode(&mut inp, user_id.value(), GJWT.clone(), LAGER_USER_IDS.clone());
        }

    });

}
