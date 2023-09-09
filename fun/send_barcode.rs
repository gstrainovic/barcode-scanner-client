use fltk::dialog;
use notify_rust::Notification;
use req::write_barcode::write_barcode;

pub fn send_barcode(barcode: String, user: i32, jwt: &str, lager_user_ids: &Vec<i32>) {
    let barcode_c = barcode.clone();

    if jwt.is_empty() {
        Notification::new()
        .summary(&format!("Barcode Scanner: {} lokal gespeichert", barcode_c))
        .show()
        .unwrap();
        return;
    }

    match write_barcode(barcode, user, jwt, &lager_user_ids, false) {
        Ok(_) => {
            Notification::new()
                .summary(&format!("Barcode Scanner: {} gesendet", barcode_c))
                .show()
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            dialog::alert_default(e.to_string().as_str());
        }
    }
}