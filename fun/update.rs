use config::VERSION;
use fltk::dialog;

pub fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("gstrainovic")
        .repo_name("barcode-scanner-client")
        .bin_name("barcode_scanner.exe")
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(VERSION)
        .build()?
        .update()?;

    if status.updated() {
        let message = format!(
            "Aktualisiert zu {}. Bitte barcode_scanner.exe nochmals starten",
            status.version()
        );
        println!("{}", message);
        dialog::alert_default(&message);
        return Err(Box::new(self_update::errors::Error::Update(message)));
    } else {
        println!("Already up to date");
        return Ok(());
    }
}

// pub fn imOnline() -> bool {
//     let mut imOnline = false;
//     let mut core = Core::new().unwrap();
//     let handle = core.handle();
//     let client = Client::configure()
//         .connector(HttpsConnector::new(4, &handle).unwrap())
//         .build(&handle);

//     let uri = "https://www.google.com";
//     let mut req = Request::new(Method::Get, uri.parse().unwrap());
//     req.headers_mut().set(ContentType::json());
//     let work = client.request(req).and_then(|res| {
//         println!("Response: {}", res.status());
//         println!("Headers: \n{}", res.headers());
//         res.body().for_each(|chunk| {
//             io::stdout()
//                 .write_all(&chunk)
//                 .map_err(From::from)
//         })
//     });
//     core.run(work).unwrap();
//     imOnline
// }