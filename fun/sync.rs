use req::write_barcode::write_barcode;
use req::check_duplicate_barcode::is_barcode_duplicate;

pub fn sync(jwt : String) {
    let history_to_sync = sqlite::get_sync_history();
    for history in history_to_sync {
        // print the history fields
        // println!(
        //     "sync: id: {}, barcode: {}, timestamp: {}, synced: {}, user_id: {}, offline: {}, lager_user_ids: {}",
        //     history.id, history.barcode, history.timestamp, history.synced, history.user_id, history.offline, history.lager_user_ids
        // );

        // let lager_user_ids: Vec<i32> = history.lager_user_ids.split(",").map(|s| s.parse().unwrap()).collect();
        let lager_user_ids: Vec<i32> = history.lager_user_ids.split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

        // println!("sync: lager_user_ids: {:?}", lager_user_ids);

        if is_barcode_duplicate(&jwt, &history.barcode, &history.user_id).unwrap() {
            // println!("sync: barcode {} is duplicate, not sendet", history.barcode);
            sqlite::update_history(history.id);
            continue;
        }

        let res = write_barcode(
            history.barcode,
            history.user_id,
            &jwt,
            &lager_user_ids,
            true,
        );
        match res {
            Ok(_) => {
                // println!("Start set id {} synced", history.id);
                sqlite::update_history(history.id);
                // println!("End set id {} synced", history.id);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}