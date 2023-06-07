pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::fs;
use crate::models::{NewHistory, History};
use std::path::Path;
use schema::history;

pub fn establish_connection() -> SqliteConnection {
    // check if db.sqlite exists, if not create it
    let db_path = Path::new("db.sqlite");
    if !db_path.exists() {
        fs::copy("db.sqlite.template", "db.sqlite").expect("Unable to copy file");
    }

    SqliteConnection::establish( "db.sqlite")
        .unwrap_or_else(|_| panic!("Error connecting to {}", "db.sqlite"))
}

pub fn create_history<'a>(conn: &mut SqliteConnection, status: &'a str, barcode: &'a str, timestamp: &'a str) -> History {

    let new_history = NewHistory {
        status,
        barcode,
        timestamp,
    };

    diesel::insert_into(history::table)
        .values(&new_history)
        .execute(conn)
        .expect("Error saving new history");

    history::table.order(history::id.desc()).first(conn).unwrap()
}

pub fn load_history(conn: &mut SqliteConnection) -> Vec<History> {
    history::table
        .order(history::id.desc())
        .limit(1000)
        .load::<History>(conn)
        .expect("Error loading history")
}