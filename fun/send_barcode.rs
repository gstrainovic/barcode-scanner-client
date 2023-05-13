use fltk::dialog;
use notify_rust::Notification;
use req::write_barcode::write_barcode;

pub fn send_barcode(barcode: String, user: i16, jwt: &str) {
    let barcode_c = barcode.clone();
    match write_barcode(barcode, user, jwt) {
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