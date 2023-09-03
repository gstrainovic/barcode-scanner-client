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
