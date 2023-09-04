pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::{fs, error::Error};
use crate::models::{NewHistory, History, User as sqliteUser};

use std::path::Path;
use schema::history::{self};
// use schema::users::{self};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
use req::loginfn::User;
use req::get_settings::Einstellungen;

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

pub fn create_history<'a>(status: &'a str, barcode: &'a str, timestamp: &'a str, nuser_id: &'a i32) -> History {
    let conn = &mut establish_connection();
    
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

pub fn load_history() -> Vec<History> {
    let conn = &mut establish_connection();

    history::table
        .order(history::id.desc())
        .limit(1000)
        .load::<History>(conn)
        .expect("Error loading history")
}

pub fn update_users(users_ar: Vec<User>) {
    use schema::users::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(users).execute(conn).unwrap();

    for user in users_ar {
        let new_user = sqliteUser {
            strapi_id: user.id,
            username: user.username,
            rolle: user.rolle,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");
    }
}

pub fn get_user(username_str: String) -> Option<sqliteUser> {
    use schema::users::dsl::*;

    let conn = &mut establish_connection();
    let user = users
        .filter(username.eq(username_str))
        .first::<sqliteUser>(conn)
        .optional()
        .expect("Error loading user");

    user
}

pub fn get_lager_users() -> Vec<sqliteUser> {
    use schema::users::dsl::*;

    let conn = &mut establish_connection();

    let lager_users = users
        .filter(rolle.eq("Lager"))
        .load::<sqliteUser>(conn)
        .expect("Error loading lager users");

    lager_users
}

pub  fn get_settings() -> Einstellungen {
    use schema::einstellungen::dsl::*;

    let conn = &mut establish_connection();

    let settings = einstellungen
        .first::<models::Einstellungen>(conn)
        .expect("Error loading settings");

    // transform to Einstellungen
    let settings = Einstellungen {
        Barcode_Mindestlaenge: settings.barcode_mindestlaenge,
        Leitcodes_Aktiv: settings.leitcodes_aktiv,
        Ausnahmen_Aktiv: settings.ausnahmen_aktiv,
    };

    settings
}

pub fn update_settings(settings: Einstellungen) {
    use schema::einstellungen::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(einstellungen).execute(conn).unwrap();

    let new_settings = models::Einstellungen {
        id: 1,
        barcode_mindestlaenge: settings.Barcode_Mindestlaenge,
        leitcodes_aktiv: settings.Leitcodes_Aktiv,
        ausnahmen_aktiv: settings.Ausnahmen_Aktiv,
    };


    diesel::insert_into(einstellungen)
        .values(&new_settings)
        .execute(conn)
        .expect("Error saving new settings");
    
}