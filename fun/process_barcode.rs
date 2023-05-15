use fltk::{input, prelude::{WidgetExt, InputExt}};
use notify_rust::Notification;
use req::get_ausnahmen::get_ausnahmen;

use crate::send_barcode::send_barcode;

// global array for barcode history
static mut BARCODES: Vec<String> = Vec::new();

pub fn process_barcode(i: &mut input::Input, user_id: String, jwt: String, lager_user_ids: Vec<i16>) {
    i.activate();
    let barcode = i.value();
    let barcode_c = barcode.clone();
    i.set_value("");

    let barcode_lower = barcode.to_lowercase();

        // print the ausnahmen
    let ausnahmen = get_ausnahmen(&jwt);
    println!("Ausnahmen: {:?}", ausnahmen);

    // get barcodes from ausnahmen
    // Ausnahmen: Ok(AusnahmenData { data: [IdAtrAusnahmen { id: 1, attributes: Ausnahmen { Barcode: "0101080", Bedeutung: "Kosmische Strahlung" } }, IdAtrAusnahmen { id: 2, attributes: Ausnahmen {
    // Barcode: "0101090", Bedeutung: "Vulkanausbruch" } }] })
    let mut barcode_ausnahmen = Vec::new();
    for ausnahme in ausnahmen.unwrap().data {
        barcode_ausnahmen.push(ausnahme.attributes.Barcode);
    }

    // print the barcodes
    println!("Barcodes: {:?}", barcode_ausnahmen);

    // if barcode ends with a string from barcode_ausnahmen, then send it directly to server
    for barcode_ausnahme in barcode_ausnahmen {
        if barcode_lower.ends_with(barcode_ausnahme.to_lowercase().as_str()) {
            send_barcode(barcode_c, user_id, &jwt, lager_user_ids);
            return;
        }
    }


    // ups express like
    // 42096242 // len 8
    // but allow
    if barcode_lower.len() < 9
    {
        Notification::new()
            .summary(&format!(
                "Barcode Scanner: {} ist zu kurz, nicht gesendet",
                barcode_c
            ))
            .show()
            .unwrap();
        return;
    }

    let f = barcode_lower.chars().nth(0).unwrap();
    let s = barcode_lower.chars().nth(1).unwrap();

    // DHL Leitcode like
    // ¨C140327619348`99000900190051
    // ¨C140327628203`99000900033018
    // 0327642113+99..

    println!("barcode: {} len: {}", barcode_lower, barcode_lower.len());
    println!("barcode contains: {}", barcode_lower.contains('+'));

    if barcode_lower.len() > 13 {
        let apostrophe = barcode_lower.chars().nth(14).unwrap();
        if (f == '¨' && s == 'c' && apostrophe == '`') || barcode_lower.contains('+') {
            Notification::new()
                .summary(&format!(
                    "Barcode Scanner: {} als DHL Leitcode erkannt, nicht gesendet",
                    barcode_c
                ))
                .show()
                .unwrap();
            return;
        }
    }

    // duplicate check
    unsafe {
        if !BARCODES.contains(&barcode_lower) {
            BARCODES.push(barcode_lower.clone());
            send_barcode(barcode_c, user_id, &jwt, lager_user_ids);
        } else {
            Notification::new()
                .summary(&format!(
                    "Barcode Scanner: {} wurde bereits gesendet",
                    barcode_c
                ))
                .show()
                .unwrap();
            return;
        }
    }
}


