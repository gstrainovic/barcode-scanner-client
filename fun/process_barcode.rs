use fltk::{
    input,
    prelude::{BrowserExt, InputExt, WidgetExt},
};
use notify_rust::Notification;
use req::{get_ausnahmen::get_ausnahmen, get_settings::get_settings, get_leitcodes::get_leitcodes, get_leitcodes::Leitcode};
use sqlite::{create_history, establish_connection};

use crate::{errors, send_barcode::send_barcode, ERROR_STATUS};

// global array for barcode history
static mut BARCODES: Vec<String> = Vec::new();

pub fn history_add(
    status: errors::Error,
    barcode_c: &str,
    mut history: fltk::browser::HoldBrowser,
) {
    let utc_time_string = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    history.add(&format!(
        "{}\t{}\t{}",
        status.message, barcode_c, utc_time_string
    ));
    history.top_line(history.size());
    unsafe { ERROR_STATUS = status.status };

    // save also to sqlite
    create_history(
        &mut establish_connection(),
        &status.message,
        &barcode_c,
        &utc_time_string,
    );
}

pub fn process_barcode(
    i: &mut input::Input,
    user_id: String,
    jwt: String,
    lager_user_ids: Vec<i16>,
    history: fltk::browser::HoldBrowser,
) {
    i.activate();
    let barcode = i.value();
    let barcode_c = barcode.clone();
    i.set_value("");

    let barcode_lower = barcode.to_lowercase();

    let settings = get_settings(&jwt).unwrap().data.attributes;
    // println!("settings{:?}", settings);

    if settings.Ausnahmen_Aktiv {
        let ausnahmen = get_ausnahmen(&jwt);
        // println!("ausnahmen{:?}", ausnahmen);

        // if barcode ends with a string from barcode_ausnahmen, then send it directly to server
        for barcode_ausnahme in ausnahmen.unwrap().data {
            if barcode_lower.ends_with(barcode_ausnahme.attributes.Barcode.to_lowercase().as_str())
            {
                send_barcode(barcode_c.clone(), user_id, &jwt, lager_user_ids);
                history_add(
                    errors::ausnahme(barcode_ausnahme.attributes.Bedeutung),
                    &barcode_c,
                    history,
                );
                return;
            }
        }
    }

    if barcode_lower.len() < settings.Barcode_Mindestlaenge as usize {
        Notification::new()
            .summary(&format!(
                "Barcode Scanner: {} ist zu kurz, nicht gesendet",
                barcode_c
            ))
            .show()
            .unwrap();
        history_add(errors::zu_kurz(), &barcode_c, history);
        return;
    }

    if settings.Leitcodes_Aktiv {
        // block DHL Leitcode like
        // ¨C140327619348`99000900190051
        // ¨C140327628203`99000900033018$
        // 0327642113+99..

        let leitcodes = get_leitcodes(&jwt).unwrap();
        println!("leitcodes{:?}", leitcodes);

        // for leitcode in leitcodes {

        //     if barcode_lower.starts_with(leitcode.attributes.Barcode.to_lowercase().as_str()) {
        //         Notification::new()
        //             .summary(&format!(
        //                 "Barcode Scanner: {} als {} Leitcode erkannt, nicht gesendet",
        //                 barcode_c, leitcode.attributes.Bedeutung
        //             ))
        //             .show()
        //             .unwrap();
        //         history_add(errors::leitcode(leitcode.attributes.Bedeutung), &barcode_c, history);
        //         return;
        //     }
        // }

        if barcode_lower.len() > 14 {
            let f = barcode_lower.chars().nth(0).unwrap();
            let s = barcode_lower.chars().nth(1).unwrap();
            let plus = barcode_lower.chars().nth(10).unwrap();
            let apostrophe = barcode_lower.chars().nth(14).unwrap();
            if (f == '¨' && s == 'c' && apostrophe == '`') || plus == '+' {
                Notification::new()
                    .summary(&format!(
                        "Barcode Scanner: {} als DHL Leitcode erkannt, nicht gesendet",
                        barcode_c
                    ))
                    .show()
                    .unwrap();
                history_add(errors::dhl_leitcode(), &barcode_c, history);
                return;
            }
        }
    }

    // duplicate check
    unsafe {
        if !BARCODES.contains(&barcode_lower) {
            BARCODES.push(barcode_lower.clone());
            send_barcode(barcode_lower, user_id, &jwt, lager_user_ids);
            history_add(errors::ok(), &barcode_c, history);
        } else {
            Notification::new()
                .summary(&format!(
                    "Barcode Scanner: {} wurde bereits gesendet",
                    barcode_c
                ))
                .show()
                .unwrap();

            history_add(errors::bereits_gesendet(), &barcode_c, history);
            return;
        }
    }
}
