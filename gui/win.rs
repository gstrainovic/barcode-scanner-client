use fltk::{
    app::screen_size,
    dialog, image,
    prelude::{GroupExt, WidgetExt, WindowExt},
    window,
};
use notify_rust::Notification;

pub fn win() -> window::Window {
    let w = screen_size().0 as i32;
    let h = screen_size().1 as i32;

    let mut win = window::Window::default().with_size(w, h);
    win.set_label("BarcodeScanner");
    win.set_callback(|w| {
        let choice = dialog::choice2_default("Barcodescanner beenden?", "Nein", "Ja", "Abbruch");
        println!("{:?}", choice);
        if choice == Some(1) {
            let mut notif = Notification::new();
            notif.summary("Barcode Scanner: Barcodescanner beendet");
            notif.show().unwrap();
            w.hide();
        }
    });

    win.make_resizable(true);

    // add icon
    let image = image::PngImage::load("gui/gravurzeile-favicon-32x32.png").unwrap();
    win.set_icon(Some(image));
    win
}
