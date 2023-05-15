use fltk::dialog;
use notify_rust::Notification;
use req::write_barcode::write_barcode;

pub fn send_barcode(barcode: String, user: String, jwt: &str, lager_user_ids: Vec<i16>) {
    let barcode_c = barcode.clone();
    match write_barcode(barcode, user, jwt, lager_user_ids) {
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