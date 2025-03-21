use diesel::prelude::*;
use super::schema::history;
use super::schema::users;
use super::schema::einstellungen;
use super::schema::ausnahmen;
use super::schema::leitcodes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct History {
    pub id: i32,
    pub status: String,
    pub barcode: String,
    pub timestamp: String,
    pub synced: bool,
    pub user_id: i32,
    pub offline: bool,
    pub lager_user_ids: String,
}

#[derive(Insertable)]
#[diesel(table_name = history)]
pub struct NewHistory<'a> {
    pub status: &'a str,
    pub barcode: &'a str,
    pub timestamp: &'a str,
    pub synced: &'a bool,
    pub user_id: &'a i32,
    pub offline: bool,
    pub lager_user_ids: &'a str,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub strapi_id: i32,
    pub username: String,
    pub rolle: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = einstellungen)]
pub struct Einstellungen {
    pub id: i32,
    pub barcode_mindestlaenge: i32,
    pub leitcodes_aktiv: bool,
    pub ausnahmen_aktiv: bool,
}

#[derive(Queryable)]
#[diesel(table_name = ausnahmen)]
pub struct Ausnahmen {
    pub id: i32,
    pub barcode: String,
    pub bedeutung: String,
}

#[derive(Insertable)]
#[diesel(table_name = ausnahmen)]
pub struct NewAusnahmen<'a> {
    pub barcode: &'a str,
    pub bedeutung: &'a str,
}

pub struct LeitcodeBuchstabe {
    pub id: i16,
    pub buchstabe: String,
    pub position: i32,
}

pub struct LeitcodesTemp {
    pub id: i16,
    pub beschreibung: String,
    pub mindeslaenge: i32,
    pub produktion: bool,
    pub leitcode_buchstabe: Vec<LeitcodeBuchstabe>,
}

#[derive(Queryable)]
pub struct Leitcodes {
    pub id: i32,
    pub beschreibung: String,
    pub mindeslaenge: i32,
    pub leitcode_buchstabe: String,
    pub produktion: bool,
}
#[derive(Insertable)]
#[diesel(table_name = leitcodes)]
pub struct NewLeitcodes<'a> {
    pub beschreibung: &'a str,
    pub mindeslaenge: &'a i32,
    pub leitcode_buchstabe: &'a str,
    pub produktion: &'a bool,
}


