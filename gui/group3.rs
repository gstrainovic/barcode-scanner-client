use fltk::{group, button, output, input, prelude::{WidgetExt, GroupExt, BrowserExt}, browser::{HoldBrowser, Browser}};
use fun::process_barcode::process_barcode;
use sqlite::load_history;
use crate::{logo_and_version, LAGER_USER_IDS, GJWT, USER_ID};

pub fn group3(
    wizard: group::Wizard,
    mut mitarbeiter1_output: output::Output,
    mut mitarbeiter2_output: output::Output,
    mut rolle_output: output::Output,
    mut benutzername_output: output::Output,
    mut barcode_input: input::Input,
    mut history_browser: HoldBrowser,
) -> (
) {
    let left_widtd_columns = 7;
    let left_offset = 3;

    let grp2 = group::Group::default().size_of(&wizard);

    let mut grid = logo_and_version();
    grid.set_layout(24, 24);

    grid.insert_ext(&mut rolle_output, 8, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut benutzername_output, 7, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut mitarbeiter1_output, 10, left_offset, left_widtd_columns, 1);
    grid.insert_ext(&mut mitarbeiter2_output, 11, left_offset, left_widtd_columns, 1);

    let mut abmelden_button = button::Button::default().with_label("Abmelden");
    grid.insert_ext(&mut abmelden_button, 13, left_offset, left_widtd_columns, 1);

    grid.insert_ext(&mut barcode_input, 15, left_offset, left_widtd_columns, 1);

    let mut senden_button = button::ReturnButton::default().with_label("Senden");
    grid.insert_ext(&mut senden_button, 17, left_offset, left_widtd_columns, 1);

    let mut header = Browser::default();
    header.add("Status\tBarcode\tZeitstempel");
    header.set_column_widths([120, 380, 100].as_ref());
    header.set_column_char('\t');

    let right_side_columns = 12;
    let right_offset = left_offset + left_widtd_columns + 1;
    grid.insert_ext(&mut header, 7, right_offset, right_side_columns, 1);

    history_browser.set_column_widths([120, 380, 100].as_ref());
    history_browser.set_column_char('\t');
    grid.insert_ext(&mut history_browser, 8, right_offset, right_side_columns, 10);

    let _ = barcode_input.take_focus();

    grp2.end();
    
    abmelden_button.set_callback(move |_| { 
        wizard.clone().prev();
        wizard.clone().prev();
        }
    );

    // load the first 1000 entries from the history table into the history browser
    let history = load_history();
    for h in history {
        history_browser.add(&format!("{}\t{}\t{}", h.status, h.barcode, h.timestamp));
        history_browser.top_line(history_browser.size());
    }

    senden_button.set_callback(move |_| {
        unsafe {
            process_barcode(&mut barcode_input, USER_ID.clone(), GJWT.clone(), &LAGER_USER_IDS, history_browser.clone());
        }

    });

}
