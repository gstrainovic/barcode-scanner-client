use crate::{
    get_hwnd_barcode_scanner::get_hwnd_barcode_scanner, group0::group0, group1::group1,
    group2::group2, group3::group3, hide_console_windows::hide_console_window,
    logo_and_version::logo_and_version, win::win,
};
use config::STRAPI_URL;
use fltk::{
    app, dialog, group, input,
    menu::Choice,
    output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetExt},
};
use fltk_theme::{ThemeType, WidgetTheme};
use fun::{looper::looper, process_barcode::process_barcode, update::update};
use notify_rust::Notification;
use req::{
    get_lager_users::get_lager_users,
    loginfn::{loginfn, User, JWT},
};
mod get_hwnd_barcode_scanner;
mod group0;
mod group1;
mod group2;
mod group3;
mod hide_console_windows;
mod logo_and_version;
mod win;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

static mut LAGER_USER_IDS: Vec<i16> = Vec::new();
static mut GJWT : String = String::new();

fn main() {
    println!("STRAPI_URL: {}", STRAPI_URL);
    hide_console_window();
    update().unwrap();

    let mut user_id = output::Output::default();

    let mut m1 = output::Output::default().with_label("Mitarbeiter 1");
    let mut m2 = output::Output::default().with_label("Mitarbeiter 2");
    let mut rf = output::Output::default().with_label("Rolle");
    let mut bf = output::Output::default().with_label("Benutzername");
    let mut inp = input::Input::default().with_label("Barcode:");
    let mut lager_choice1 = Choice::default();
    lager_choice1.add_choice("-");
    let mut lager_choice2 = Choice::default();
    lager_choice2.add_choice("-");

    let hwnd_of_barcode_scanner = get_hwnd_barcode_scanner();

    if hwnd_of_barcode_scanner != std::ptr::null_mut() {
        let message = "Barcodescanner läuft bereits!";
        println!("{}", message);
        dialog::alert_default(message);
        return;
    }

    let a = app::App::default().with_scheme(app::Scheme::Gleam);
    app::set_visible_focus(true);

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let mut win = win();

    let mut wizard = group::Wizard::default().with_size(win.width(), win.height());

    let chce = group0(wizard.clone());

    group1(
        wizard.clone(),
        lager_choice1.clone(),
        lager_choice2.clone(),
        m1.clone(),
        m2.clone(),
        bf.clone(),
        rf.clone(),
        user_id.clone(),
        inp.clone(),
        chce,
    );
    group2(
        wizard.clone(),
        m1.clone(),
        m2.clone(),
        lager_choice1.clone(),
        lager_choice2.clone(),
    );
    group3(
        wizard.clone(),
        m1.clone(),
        m2.clone(),
        user_id,
        rf.clone(),
        bf.clone(),
        inp.clone(),
    );

    wizard.end();

    win.end();
    win.show();
    win.activate();

    unsafe {
        winapi::um::winuser::ShowWindow(hwnd_of_barcode_scanner, winapi::um::winuser::SW_MAXIMIZE);
        winapi::um::winuser::SetForegroundWindow(hwnd_of_barcode_scanner);
        winapi::um::winuser::SetActiveWindow(hwnd_of_barcode_scanner);
    }

    a.run().unwrap();
}
