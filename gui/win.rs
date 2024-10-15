use fltk::{
    dialog, image,
    prelude::{GroupExt, WidgetExt, WindowExt},
    window::{self, Window},
    app
};
use notify_rust::Notification;
use crate::favicon::FAVICON;

pub fn win() -> window::Window {
    let (screen_width, screen_height) = app::screen_size();
    let mut win = Window::default()
        .with_size(screen_width as i32, (screen_height) as i32);
    
    win.set_label("BarcodeScanner");
    win.set_callback(|w| {
        let choice = dialog::choice2_default("Barcodescanner beenden?", "Nein", "Ja", "Abbruch");
        if choice == Some(1) {
            let mut notif = Notification::new();
            notif.summary("Barcode Scanner: Barcodescanner beendet");
            notif.show().unwrap();
            w.hide();
        }
    });

    win.make_resizable(true);

    let image = image::SvgImage::from_data(FAVICON).unwrap();
    win.set_icon(Some(image));
    win
}
