pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::{fs, error::Error};
use crate::models::{NewHistory, History, User};
use std::path::Path;
use schema::history::{self};
use schema::users::{self};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");


fn run_migrations<DB: diesel::backend::Backend>(connection: &mut impl MigrationHarness<DB>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    let path = Path::new("db.sqlite");

    if !path.exists() {
        fs::File::create(path).expect("Unable to create file");
    }

    let mut conn = SqliteConnection::establish( path.to_str().unwrap() )
        .unwrap_or_else(|_| panic!("Error connecting to {}", path.to_str().unwrap()));

    run_migrations(&mut conn).unwrap();

    conn
}

pub fn create_history<'a>(conn: &mut SqliteConnection, status: &'a str, barcode: &'a str, timestamp: &'a str, nuser_id: &'a i32) -> History {
    let new_history = NewHistory {
        status,
        barcode,
        timestamp,
        synced: &false,
        user_id: nuser_id,
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

pub fn update_users(conn: &mut SqliteConnection, users_ar: Vec<User>) {
    use schema::users::dsl::*;

    diesel::delete(users).execute(conn).unwrap();

    for user in users_ar {
        diesel::insert_into(users)
            .values(&user)
            .execute(conn)
            .expect("Error saving new user");
    }
}