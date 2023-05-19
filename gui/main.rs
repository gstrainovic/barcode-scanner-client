use crate::{
    get_hwnd_barcode_scanner::get_hwnd_barcode_scanner, group0::group0, group1::group1,
    group2::group2, group3::group3, hide_console_windows::hide_console_window,
    logo_and_version::logo_and_version, win::win, device_choice::device_choice,
    
};
use fltk::{
    app, dialog, group, input,
    menu::Choice,
    output,
    prelude::{GroupExt, MenuExt, WidgetExt}, browser::HoldBrowser,
};
use fltk_theme::{ThemeType, WidgetTheme};
use fun::{update::update};

mod get_hwnd_barcode_scanner;
mod group0;
mod group1;
mod group2;
mod group3;
mod hide_console_windows;
mod logo_and_version;
mod win;
mod logo;
mod favicon;
mod device_choice;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

static mut LAGER_USER_IDS: Vec<i16> = Vec::new();
static mut GJWT : String = String::new();
static mut USER_ID : String = String::new();

fn main() {
    hide_console_window();
    update().unwrap();

    // globals
    let mitarbeiter1_output = output::Output::default().with_label("Mitarbeiter 1");
    let mitarbeiter2_output = output::Output::default().with_label("Mitarbeiter 2");
    let rolle_output = output::Output::default().with_label("Rolle");
    let benutzername_output = output::Output::default().with_label("Benutzername");
    let barcode_input = input::Input::default().with_label("Barcode:");
    let device_choice = device_choice();
    let history_browser = HoldBrowser::default();
    let mut lager_choice1 = Choice::default();
    lager_choice1.add_choice("-");
    let mut lager_choice2 = Choice::default();
    lager_choice2.add_choice("-");
    let hwnd_of_barcode_scanner = get_hwnd_barcode_scanner();
    if hwnd_of_barcode_scanner != std::ptr::null_mut() {
        dialog::alert_default("Barcodescanner läuft bereits!");
        return;
    }

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    app::set_visible_focus(true);

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let mut win = win();

    let wizard = group::Wizard::default().with_size(win.width(), win.height());

    group0(wizard.clone(), device_choice.clone());

    group1(
        wizard.clone(),
        lager_choice1.clone(),
        lager_choice2.clone(),
        mitarbeiter1_output.clone(),
        mitarbeiter2_output.clone(),
        benutzername_output.clone(),
        rolle_output.clone(),
        barcode_input.clone(),
        device_choice,
    );
    group2(
        wizard.clone(),
        mitarbeiter1_output.clone(),
        mitarbeiter2_output.clone(),
        lager_choice1.clone(),
        lager_choice2.clone(),
    );
    group3(
        wizard.clone(),
        mitarbeiter1_output.clone(),
        mitarbeiter2_output.clone(),
        rolle_output.clone(),
        benutzername_output.clone(),
        barcode_input.clone(),
        history_browser.clone(),
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

    app.run().unwrap();
}
