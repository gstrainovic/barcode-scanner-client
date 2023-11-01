use fltk::{
    input,
    prelude::{BrowserExt, InputExt, WidgetExt},
};
use notify_rust::Notification;
use req::get_settings::Einstellungen;
use req::{
    check_duplicate_barcode::is_barcode_duplicate, get_ausnahmen::get_ausnahmen,
    get_leitcodes::get_leitcodes, get_leitcodes::IdAtrBuchstaben, get_leitcodes::Leitcode,
    get_leitcodes::LeitcodeBuchstabe, get_settings::get_settings,
};
use sqlite::{
    create_history, get_ausnahmen as get_ausnahmen_sqlite, get_leitcodes_sql,
    get_settings as get_settings_sqlite, update_ausnahmen, update_leitcodes, update_settings, is_barcode_duplicate_sqlite
};

use crate::{errors, send_barcode::send_barcode, ERROR_STATUS};

pub fn history_add(
    status: errors::Error,
    barcode_c: &str,
    mut history: fltk::browser::HoldBrowser,
    nuser_id: i32,
    offline: bool,
    lager_user_ids: &Vec<i32>,
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
        &status.message,
        &barcode_c,
        &utc_time_string,
        &nuser_id,
        offline,
        lager_user_ids,
    );
}

pub fn process_barcode(
    i: &mut input::Input,
    user_id: i32,
    jwt: String,
    lager_user_ids: &Vec<i32>,
    history: fltk::browser::HoldBrowser,
) {
    i.activate();
    let barcode = i.value();
    let barcode_c = barcode.clone();
    i.set_value("");

    
    // remove barcode all non-ascii, whitespace characters and make it lowercase
    let barcode_lower = barcode_c
        .chars()
        .filter(|c| c.is_ascii() && !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();

    let mut settings = Einstellungen {
        Barcode_Mindestlaenge: 0,
        Leitcodes_Aktiv: false,
        Ausnahmen_Aktiv: false,
    };

    let offline = jwt.is_empty();

    if offline {
        settings = get_settings_sqlite()
    } else {
        settings = get_settings(&jwt).unwrap().data.attributes;
        update_settings(get_settings(&jwt).unwrap().data.attributes);
    }

    // printn the settings
    println!("settings: {:?}", settings);

    if settings.Ausnahmen_Aktiv {
        let mut ausnahmen = Vec::new();

        if offline {
            ausnahmen = get_ausnahmen_sqlite();
        } else {
            ausnahmen = get_ausnahmen(&jwt).unwrap();
            update_ausnahmen(get_ausnahmen(&jwt).unwrap());
        }

        // if barcode ends with a string from barcode_ausnahmen, then send it directly to server
        for barcode_ausnahme in ausnahmen {
            if barcode_lower.ends_with(barcode_ausnahme.Barcode.to_lowercase().as_str()) {
                send_barcode(barcode_c.clone(), user_id, &jwt, &lager_user_ids);
                history_add(
                    errors::ausnahme(barcode_ausnahme.Bedeutung),
                    &barcode_c,
                    history,
                    user_id,
                    offline,
                    lager_user_ids,
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
        history_add(
            errors::zu_kurz(),
            &barcode_c,
            history,
            user_id,
            offline,
            lager_user_ids,
        );
        return;
    }

    if settings.Leitcodes_Aktiv {
        // block DHL Leitcode like
        // ¨C140327619348`99000900190051
        // ¨C140327628203`99000900033018
        // 0327642113+99..

        let mut leitcodes = Vec::new();

        if jwt.is_empty() {
            leitcodes = get_leitcodes_sql();
        } else {
            leitcodes = get_leitcodes(&jwt).unwrap().data;
            update_leitcodes(get_leitcodes(&jwt).unwrap());
        }

        println!("leitcodes: {:?}", leitcodes);

        for idatr in leitcodes {
            let attribute: Leitcode = idatr.attributes;
            if barcode_lower.len() > attribute.Mindeslaenge as usize {
                let beschreibung = attribute.Beschreibung;
                let data_buchstaben: Vec<IdAtrBuchstaben> = attribute.Leitcode_Buchstabe.data;
                for buchstabe_atr_id in data_buchstaben {
                    let buchstabe_attr: LeitcodeBuchstabe = buchstabe_atr_id.attributes;
                    let position: usize = buchstabe_attr.Position_Null_Beginnend as usize;

                    // does the barcode match witch buchstabe at position?
                    println!("barcode_lower{:?}", barcode_lower);
                    if barcode_lower.len() > position {
                        let barcode_buchstabe = barcode_lower.chars().nth(position).unwrap();
                        println!("barcode_buchstabe{:?}", barcode_buchstabe);
                        if buchstabe_attr.Buchstabe == barcode_buchstabe.to_string() {
                            Notification::new()
                                .summary(&format!(
                                    "Barcode Scanner: {} als {} erkannt, nicht gesendet",
                                    barcode_c, beschreibung
                                ))
                                .show()
                                .unwrap();
                            history_add(
                                errors::leitcode(beschreibung),
                                &barcode_c,
                                history,
                                user_id,
                                offline,
                                lager_user_ids,
                            );
                            return;
                        }
                    }
                }
            }
        }
    }

    let mut is_barcode_duplicate_bool = false;

    if offline {
        is_barcode_duplicate_bool = is_barcode_duplicate_sqlite(&barcode_c);
    } else {
        is_barcode_duplicate_bool = is_barcode_duplicate(&jwt, &barcode_c, &user_id).unwrap();
    }

    if !is_barcode_duplicate_bool {
        send_barcode(barcode_lower.clone(), user_id, &jwt, lager_user_ids);

        // does the barcode contain a number?
        let mut contains_number = false;
        for c in barcode_lower.chars() {
            if c.is_numeric() {
                contains_number = true;
                break;
            }
        }

        let err = if contains_number {
            errors::ok()
        } else {
            errors::no_numbers()
        };

        history_add(err, &barcode_c, history, user_id, offline, lager_user_ids);
    } else {
        Notification::new()
            .summary(&format!(
                "Barcode Scanner: {} wurde bereits gesendet",
                barcode_c
            ))
            .show()
            .unwrap();

        history_add(
            errors::bereits_gesendet(),
            &barcode_c,
            history,
            user_id,
            offline,
            lager_user_ids,
        );
        return;
    }
}
