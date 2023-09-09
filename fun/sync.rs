use req::write_barcode::write_barcode;

pub fn sync(jwt : String) {
    let history_to_sync = sqlite::get_sync_history();
    for history in history_to_sync {
        // print the history fields
        println!(
            "id: {}, barcode: {}, timestamp: {}, synced: {}, user_id: {}, offline: {}, lager_user_ids: {}",
            history.id, history.barcode, history.timestamp, history.synced, history.user_id, history.offline, history.lager_user_ids
        );

        let lager_user_ids: Vec<i32> = history.lager_user_ids.split(",").map(|s| s.parse().unwrap()).collect();

        println!("lager_user_ids: {:?}", lager_user_ids);

        let res = write_barcode(
            history.barcode,
            history.user_id,
            &jwt,
            &lager_user_ids,
            true,
        );
        match res {
            Ok(_) => {
                println!("Start set id {} synced", history.id);
                sqlite::update_history(history.id);
                println!("End set id {} synced", history.id);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}