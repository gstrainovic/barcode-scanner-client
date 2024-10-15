use config::VERSION;

pub fn update() {
    if let Ok(update) = self_update::backends::github::Update::configure()
        .repo_owner("gstrainovic")
        .repo_name("barcode-scanner-client")
        .bin_name("barcode_scanner.exe")
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(VERSION)
        .build()
    {
        if let Ok(status) = update.update() {
            if status.updated() {
                let message = format!(
                    "Aktualisiert zu {}. Bitte barcode_scanner.exe nochmals starten",
                    status.version()
                );
                // Optional: Zeigen Sie eine Nachricht an, wenn das Update erfolgreich war
                fltk::dialog::alert_default(&message);
            }
        }
    }
}