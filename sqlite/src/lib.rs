pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::{fs, error::Error};
use crate::models::{NewHistory, History, User as sqliteUser, Ausnahmen as sqliteAusnahmen, Leitcodes as sqliteLeitcodes};

use std::path::Path;
use schema::history::{self};
// use schema::users::{self};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
use req::loginfn::User;
use req::get_settings::Einstellungen;
use req::get_ausnahmen::Ausnahmen as reqAusnahmen;
use req::get_leitcodes::Leitcode as reqLeitcodes;
use req::get_leitcodes::Data as reqLeitcodeData;
use req::get_leitcodes::LeitcodeBuchstabe;
use req::get_leitcodes::DataBuchstaben;
use req::get_leitcodes::IdAtrBuchstaben;
use req::get_leitcodes::IdAtr;
use schema::ausnahmen::{self};
use schema::leitcodes::{self};

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

pub fn create_history<'a>(status: &'a str, barcode: &'a str, timestamp: &'a str, nuser_id: &'a i32, offline: bool, lager_user_ids: &Vec<i32>)  {
    let conn = &mut establish_connection();

    let lager_user_ids_string = lager_user_ids.into_iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    
    let new_history = NewHistory {
        status,
        barcode,
        timestamp,
        synced: &false,
        user_id: nuser_id,
        offline: offline,
        lager_user_ids: &lager_user_ids_string,
    };

    diesel::insert_into(history::table)
        .values(&new_history)
        .execute(conn)
        .expect("Error saving new history");

    // history::table.order(history::id.asc()).first(conn).unwrap()
}

pub fn is_barcode_duplicate_sqlite(barcode_string: &str) -> bool {
    use schema::history::dsl::*;

    let conn = &mut establish_connection();

    let history_rec = history
        .filter(barcode.eq(barcode_string)) // change this line
        .first::<History>(conn)
        .optional()
        .expect("Error loading history");

    if history_rec.is_some() {
        true
    } else {
        false
    }
}

pub fn load_history() -> Vec<History> {
    let conn = &mut establish_connection();

    history::table
        .order(history::id.asc())
        .limit(500)
        .load::<History>(conn)
        .unwrap_or_else(|error| {
            eprintln!("Error loading history: {}", error);
            panic!("Failed to load history");
        })
}

pub fn update_history(idi: i32) {
    use schema::history::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(history.find(idi))
        .set(synced.eq(true))
        .execute(conn)
        .unwrap();
}

pub fn get_sync_history() -> Vec<History> {
    let conn = &mut establish_connection();

    let history_rec = history::table
        .filter(history::synced.eq(false))
        .filter(history::offline.eq(true))
        .filter(history::status.eq("OK"))
        .load::<History>(conn)
        .expect("Error loading history");

    history_rec
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

pub fn get_settings() -> Einstellungen {
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

pub fn update_ausnahmen(ausnahmen_rec: Vec<reqAusnahmen>) {
    use schema::ausnahmen::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(ausnahmen).execute(conn).unwrap();

    for ausnahme in ausnahmen_rec {
        let new_ausnahme = models::NewAusnahmen {
            barcode: &ausnahme.Barcode,
            bedeutung: &ausnahme.Bedeutung,
        };

        diesel::insert_into(ausnahmen)
            .values(&new_ausnahme)
            .execute(conn)
            .expect("Error saving new ausnahme");
    }
}

pub fn get_ausnahmen() -> Vec<reqAusnahmen> {
    let conn = &mut establish_connection();

    let ausnahmen_rec = ausnahmen::table
        .load::<sqliteAusnahmen>(conn)
        .expect("Error loading ausnahmen");

    let mut ausnahmen: Vec<reqAusnahmen> = Vec::new();

    for ausnahme in ausnahmen_rec {
        let ausnahme = reqAusnahmen {
            // id: ausnahme.id,
            Barcode: ausnahme.barcode,
            Bedeutung: ausnahme.bedeutung,
        };

        ausnahmen.push(ausnahme);
    }

    ausnahmen
}

pub fn update_leitcodes(leitcodes_req_data: reqLeitcodeData) {
    use schema::leitcodes::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(leitcodes).execute(conn).unwrap();

    let id_atr_ar = leitcodes_req_data.data;

    for id_atr in id_atr_ar {
        let attributes = id_atr.attributes;

        let new_leitcode_temp = models::LeitcodesTemp {
            id: id_atr.id,
            beschreibung: attributes.Beschreibung,
            mindeslaenge: attributes.Mindeslaenge,
            produktion: attributes.Produktion,
            leitcode_buchstabe: attributes.Leitcode_Buchstabe.data.into_iter().map(|buchstabe| {
                models::LeitcodeBuchstabe {
                    id: buchstabe.id,
                    buchstabe: buchstabe.attributes.Buchstabe,
                    position: buchstabe.attributes.Position_Null_Beginnend,
                }
            }).collect(),
        };

        let mut leitcode_buchstabe_str = String::new();

        for buchstabe in new_leitcode_temp.leitcode_buchstabe {
            leitcode_buchstabe_str.push_str(&format!("{}:{};", buchstabe.buchstabe, buchstabe.position));
        }

        let new_leitcode = models::NewLeitcodes {
            beschreibung: &new_leitcode_temp.beschreibung,
            mindeslaenge: &new_leitcode_temp.mindeslaenge,
            produktion: &new_leitcode_temp.produktion,
            leitcode_buchstabe: &leitcode_buchstabe_str,
        };

        diesel::insert_into(leitcodes)
            .values(&new_leitcode)
            .execute(conn)
            .expect("Error saving new leitcode");

    }
}

pub fn get_leitcodes_sql() -> Vec<IdAtr> {
    let conn = &mut establish_connection();

    let leitcodes_rec = leitcodes::table
        .load::<sqliteLeitcodes>(conn)
        .expect("Error loading leitcodes");

    let mut leitcodes: Vec<reqLeitcodes> = Vec::new();

    for leitcode in leitcodes_rec {
        let leitcode_buchstabe_ar: Vec<&str> = leitcode.leitcode_buchstabe.split(";").collect();

        let mut leitcode_buchstabe: Vec<LeitcodeBuchstabe> = Vec::new();

        for buchstabe in leitcode_buchstabe_ar {
            let buchstabe_ar: Vec<&str> = buchstabe.split(":").collect();

            if buchstabe_ar.len() == 2 {
                let buchstabe = LeitcodeBuchstabe {
                    Buchstabe: buchstabe_ar[0].to_string(),
                    Position_Null_Beginnend: buchstabe_ar[1].parse::<i32>().unwrap(),
                };

                leitcode_buchstabe.push(buchstabe);
            }
        }

        let id_atr_buchstaben: Vec<IdAtrBuchstaben> = leitcode_buchstabe.into_iter().enumerate().map(|(i, buchstabe)| {
            IdAtrBuchstaben {
                id: i as i16,
                attributes: LeitcodeBuchstabe {
                    Buchstabe: buchstabe.Buchstabe,
                    Position_Null_Beginnend: buchstabe.Position_Null_Beginnend,
                },
            }
        }).collect();

        let leitcode = reqLeitcodes {
            Beschreibung: leitcode.beschreibung,
            Mindeslaenge: leitcode.mindeslaenge,
            Produktion: leitcode.produktion,
            Leitcode_Buchstabe: DataBuchstaben {
                data: id_atr_buchstaben,
            },
        };

        leitcodes.push(leitcode);
    }

    let id_atr = leitcodes.into_iter().enumerate().map(|(i, leitcode)| {
        IdAtr {
            id: i as i16,
            attributes: leitcode,
        }
    }).collect();

    id_atr
}  


