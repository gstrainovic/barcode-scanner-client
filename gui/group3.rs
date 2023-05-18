use fltk::{group, button, output, input, prelude::{WidgetExt, GroupExt, InputExt, MenuExt, BrowserExt, TableExt}, menu::Choice, tree, enums, browser::{HoldBrowser, Browser}};
use fltk_table::SmartTable;
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
    mut history: HoldBrowser,
) -> (
) {
    let left_widtd_columns = 7;
    let left_offset = 3;

    let grp2 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();
    // grid.debug(true);
    grid.set_layout(24, 24);

    grid.insert_ext(&mut rf, 8, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut bf, 7, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut m1, 10, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut m2, 11, left_offset, left_widtd_columns, 1);

    let mut backb = button::Button::default().with_label("Abmelden");
    grid.insert_ext(&mut backb, 13, left_offset, left_widtd_columns, 1);


    grid.insert_ext(&mut inp, 15, left_offset, left_widtd_columns, 1);

    let mut sendenb = button::ReturnButton::default().with_label("Senden");
    grid.insert_ext(&mut sendenb, 17, left_offset, left_widtd_columns, 1);

    let mut header = Browser::default();
    header.add("Status\tBarcode\tZeitstempel");
    header.set_column_widths([120, 380, 100].as_ref());
    header.set_column_char('\t');
    let right_side_columns = 12;
    let right_offset = left_offset + left_widtd_columns + 1;
    grid.insert_ext(&mut header, 7, right_offset, right_side_columns, 1);

    history.set_column_widths([120, 380, 100].as_ref());
    history.set_column_char('\t');
    grid.insert_ext(&mut history, 8, right_offset, right_side_columns, 10);

    let _ = inp.take_focus();

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
            process_barcode(&mut inp, user_id.value(), GJWT.clone(), LAGER_USER_IDS.clone(), history.clone());
        }

    });

}
