use std::sync::Arc;

use flt_rust_demo::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use fltk::{
    app, button, group,
    group::{Group, Pack, Tabs},
    window,
};
use fltk::{prelude::*, *};
use notify_rust::Notification;
use serde_json::{json, Value, Map};
use serde::Deserialize;

use fltk_theme::{ColorTheme, color_themes, ThemeType, WidgetTheme};
use fltk::{enums::*, prelude::*, *};
use fltk_theme::{SchemeType, WidgetScheme};
use fltk::{prelude::*, *};
use fltk_theme::{widget_themes};

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();


#[derive(Deserialize, Debug)]
struct JWT {
  jwt: Option<String>,
  error: Option<Map<String, Value>>,
  user: Option<User>
}

#[derive(Deserialize, Debug)]
struct User {
  username: String,
  id: i16,
  rolle: String,
}


#[derive(Deserialize, Debug)]
struct BarcodeData {
  data: IdAtr
}


#[derive(Deserialize, Debug)]
struct IdAtr {
  id: i16,
  attributes: Map<String, Value>
}

#[tokio::main]
async fn write_barcode (barcode : String, user : i16, jwt : &str) -> Result<(BarcodeData), reqwest::Error> {

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post("http://167.235.59.184:1337/api/barcodes")
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&json!({
          "data": {
            "barcode": barcode,
            "users_permissions_user": user
          }
        }))
        .send()
        .await?;

    let body = res.text().await?;
    
    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}

#[tokio::main]
async fn loginfn(user: String, pass: String) -> Result<(JWT), reqwest::Error> {

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post("http://167.235.59.184:1337/api/auth/local")
        .json(&json!({
          "identifier": user, //"gost", info@strainovic-it.ch
          "password": pass // "njM3&?HwtCe#GhV" , FBTtJ4nQC44MJir
        }))
        .send()
        .await?;

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}

fn main_themes() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::AquaClassic);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Classic|Aero|Metro|AquaClassic|Greybird|Blue|HighContrast|Dark");
    choice.set_value(3);
    choice.set_frame(widget_themes::OS_PANEL_THIN_UP_BOX);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::new(160, 220, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = match c.value() {
            0 => WidgetTheme::new(ThemeType::Classic),
            1 => WidgetTheme::new(ThemeType::Aero),
            2 => WidgetTheme::new(ThemeType::Metro),
            3 => WidgetTheme::new(ThemeType::AquaClassic),
            4 => WidgetTheme::new(ThemeType::Greybird),
            5 => WidgetTheme::new(ThemeType::Blue),
            6 => WidgetTheme::new(ThemeType::HighContrast),
            7 => WidgetTheme::new(ThemeType::Dark),
            _ => WidgetTheme::new(ThemeType::Classic),
        };
        theme.apply();
    });

    a.run().unwrap();
}

fn ccc() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_visible_focus(false);

    let color_theme = ColorTheme::new(color_themes::BLACK_THEME);
    color_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Black|Dark|Gray|Shake|Tan");
    choice.set_value(0);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    button::Button::new(160, 220, 80, 30, "Hello");
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = match c.value() {
            0 => ColorTheme::new(color_themes::BLACK_THEME),
            1 => ColorTheme::new(color_themes::DARK_THEME),
            2 => ColorTheme::new(color_themes::GRAY_THEME),
            3 => ColorTheme::new(color_themes::SHAKE_THEME),
            4 => ColorTheme::new(color_themes::TAN_THEME),
            _ => ColorTheme::new(color_themes::BLACK_THEME),
        };
        theme.apply();
    });

    a.run().unwrap();
}

fn schemes() {
    let a = app::App::default();
    let scheme = WidgetScheme::new(SchemeType::Clean);
    scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Clean|Crystal|Gleam");
    choice.set_value(3);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "Check");
    check.set_value(true);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "Round");
    round.set_value(true);
    let mut _btn = button::Button::new(160, 220, 80, 30, "Hello");
    win.end();
    win.show();
    choice.set_callback(|c| {
        let scheme = match c.value() {
            0 => WidgetScheme::new(SchemeType::Clean),
            1 => WidgetScheme::new(SchemeType::Crystal),
            2 => WidgetScheme::new(SchemeType::Gleam),
            _ => unimplemented!(),
        };
        scheme.apply();
    });
    a.run().unwrap();
}

fn main() {
    hide_console_window();
    find_device();

    let w = 640;
    let h = 480;

    // let a = app::App::default();
       let a = app::App::default().with_scheme(app::Scheme::Gleam);
       app::set_visible_focus(true);
 
      let widget_theme = WidgetTheme::new(ThemeType::Dark);
      widget_theme.apply();

    let mut win = window::Window::default().with_size(w, h);
    win.set_label("BarcodeScanner");
    win.set_callback(|w| {
        let choice = dialog::choice2_default("Barcodescanner beenden?", "Nein", "Ja", "Abbruch");
        println!("{:?}", choice);
        if choice == Some(1) {
            let mut notif = Notification::new();
            notif.summary("Barcodescanner beendet");
            notif.show().unwrap();
            w.hide();
        }
    });

    let mut wizard = group::Wizard::default().with_size(w, h);

    let grp1 = group::Group::default().size_of(&wizard);

    let col = group::Flex::default_fill().column();
    frame::Frame::default();

    let mut mp = group::Flex::default().row();

    frame::Frame::default();

    // Takes a path
    let mut frame = frame::Frame::default();
    let mut myimage = image::SvgImage::load("gravurzeile.svg").unwrap();
    myimage.scale(200, 200, true, true);
    frame.set_image(Some(myimage));

    let spacer = frame::Frame::default();

    let mut bp = group::Flex::default().column();

    frame::Frame::default();
    let mut wf = frame::Frame::default().with_label("Bitte anmelden");
    wf.set_label_size(20);

    let username = input::Input::default();
    let password = input::SecretInput::default();

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Benutzername:")
            .with_align(enums::Align::Inside | enums::Align::Right);

        urow.set_size(&username, 180);
        urow.add(&username);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Passwort:")
            .with_align(enums::Align::Inside | enums::Align::Right);

        prow.set_size(&password, 180);
        prow.add(&password);
        prow.end();
    }


    let pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    frame::Frame::default();
    let mut login = create_button("Anmelden");
    brow.set_size(&login, 180);
    brow.end();

    let b = frame::Frame::default();

    frame::Frame::default();

    bp.set_size(&wf, 60);
    bp.set_size(&urow, 30);
    bp.set_size(&prow, 30);
    bp.set_size(&pad, 1);
    bp.set_size(&brow, 30);
    bp.set_size(&b, 30);

    bp.end();

    frame::Frame::default();

    mp.set_size(&frame, 200);
    mp.set_size(&spacer, 10);
    mp.set_size(&bp, 300);

    mp.end();

    frame::Frame::default();

    col.end();

    grp1.end();


    let mut grp2 = group::Group::default().size_of(&wizard);

    let mut bf = output::Output::new(150, 150, 150, 30, "Benutername");
    let mut backb = button::Button::new(320, 150, 150, 30, "Abmelden");
    let mut rf = output::Output::new(150, 200, 150, 30, "Rolle");

    let mut inp = input::Input::default()
        .with_label("Barcode:")
        .with_size(320, 30)
        .with_pos(150, 250);
    inp.set_trigger(enums::CallbackTrigger::EnterKey);

    let mut sendenb = button::ReturnButton::new(150, 320, 320, 30, "Senden");


    grp2.add(&bf);
    grp2.add(&rf);
    grp2.end();

    wizard.end();

    backb.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });

    let mut guser  = None;
    let mut gjwt = String::new();

    login.set_callback(move |_| {
        let res = loginfn(username.value(), password.value());
        println!("{:?}", res);

        match res {
            Ok(j) => {
              match j {
                JWT { user, jwt, error: None } => {
                  guser = user;
                  gjwt = jwt.unwrap();
                  wizard.next();
                },
                JWT {user, jwt: None, error: Some(err) } => {
                  println!("Error err: {:?}", err);
                  match err.get_key_value("message") {
                    Some((k, v)) => {
                      let value_s = v.as_str().unwrap();
                      match value_s {
                        "Invalid identifier or password" => {
                          println!("{}", value_s);
                          dialog::alert_default("Benutzername oder Passwort falsch");
                        },
                       "password is a required field" => {
                          println!("{}", value_s);
                          dialog::alert_default("Passwort ist ein Pflichtfeld");
                        },
                       "username is a required field" => {
                          println!("{}", value_s);
                          dialog::alert_default("Benutzer ist ein Pflichtfeld");
                        },
                        "2 errors occurred" => {
                          println!("{}", value_s);
                          dialog::alert_default("Benutzername und Passwort sind Pflichtfelder");
                        },
                        _ =>  {
                          println!("Error2: {:?}", value_s);
                          dialog::alert_default(value_s);
                        }
                      }
                      println!("Error: {} {}", k, v);
                    },
                    None => {
                      println!("Error: {:?}", err);
                    }
                  }
                },
                _ => {
                  println!("Error j : {:?}", j);
                }
              }
              }
            Err(e) => {
                println!("Error e: {}", e);
            }
        }

        println!("User: {:?}", guser);
        println!("JWT: {:?}", gjwt);

        let username = guser.as_ref().unwrap().username.clone();

        bf.set_value(&username);
        rf.set_value(guser.as_ref().unwrap().rolle.as_str());

        let user_id = guser.as_ref().unwrap().id;

        let jwt = gjwt.clone();

        inp.set_callback(move |i| { 


            // send notification
            let mut notif = Notification::new();
            notif.summary("Anmeldung");
            notif.body(&format!("{} hat sich angemeldet", username));
            notif.show().unwrap();

            // let iii = user.id.clone();

            process_barcode(i, user_id, &jwt);
        });



        // start looper in new thread
        std::thread::spawn(|| looper());
    });



    win.end();
    win.show();
    // win.maximize();
    win.activate();

    a.run().unwrap();
}

fn create_button(caption: &str) -> button::ReturnButton {
    let mut btn = button::ReturnButton::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn.set_size(500, 30);
    btn
}

fn process_barcode(i: &mut input::Input, user: i16, jwt: &str) {
    // set focus to input field
    i.activate();
    let barcode = i.value();

    write_barcode(i.value(), user, jwt);

    // remove value from input
    println!("Barcode: {}", barcode);
    let mut notif = Notification::new();
    notif.summary("Barcode");
    notif.body(&barcode);
    notif.show().unwrap();
    i.set_value("");
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn find_device() -> RawInputManager {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    let devices = manager.get_device_list();

    // println!("Devices: {:?}", devices);

    let keyboards = Arc::new(devices.keyboards);

    // println!("Keyboards: {:?}", keyboards);

    // filter keyboard which contains 'VID_0483&PID_5750', send alert if not found
    let keyboard = keyboards
        .iter()
        .find(|k| k.name.contains("VID_0483&PID_5750"))
        .unwrap_or_else(|| {
            // dialog::alert_default("Barcodescanner nicht gefunden, bitte anstecken und einschalte und Programm erneut starten");
            println!("Keyboard not found");
            dialog::alert_default("Barcodescanner nicht gefunden. Bitte anschliessen, einschalten und Programm neu starten");
            std::process::exit(1);
        });
    // println!("Keyboard: {:?}", keyboard);
    // gf.set_label(&keyboard.name);

    manager.filter_devices(vec![keyboard.name.clone()]);
    return manager;
}

fn looper() {
    let mut manager = find_device();

    let mut switch_back_hwd = unsafe { winapi::um::winuser::GetForegroundWindow() };

    loop {
        // handle events
        if let Some(event) = manager.get_event() {
            println!("Event: {:?}", event);

            let my_windows_hwnd = unsafe {
                winapi::um::winuser::FindWindowA(
                    std::ptr::null(),
                    "BarcodeScanner\0".as_ptr() as *const i8,
                )
            };

            let current_active_window_hwnd = unsafe { winapi::um::winuser::GetForegroundWindow() };

            if current_active_window_hwnd != my_windows_hwnd {
                switch_back_hwd = current_active_window_hwnd;
            }

            unsafe {
                winapi::um::winuser::ShowWindow(my_windows_hwnd, winapi::um::winuser::SW_MAXIMIZE);
                winapi::um::winuser::SetForegroundWindow(my_windows_hwnd);
                winapi::um::winuser::SetActiveWindow(my_windows_hwnd);
            }

            match event {
                RawEvent::KeyboardEvent(_, KeyId::Return, State::Released) => {
                    // activate the window current_active_window_hwnd again
                    unsafe {
                        winapi::um::winuser::ShowWindow(
                            my_windows_hwnd,
                            winapi::um::winuser::SW_MINIMIZE,
                        );
                        winapi::um::winuser::SetForegroundWindow(switch_back_hwd);
                        winapi::um::winuser::SetActiveWindow(switch_back_hwd);
                    }
                }

                _ => {}
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
