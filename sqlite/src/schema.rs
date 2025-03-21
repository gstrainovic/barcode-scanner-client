// @generated automatically by Diesel CLI.

diesel::table! {
    history (id) {
        id -> Integer,
        status -> Text,
        barcode -> Text,
        timestamp -> Text,
        synced -> Bool,
        user_id -> Integer,
        offline -> Bool,
        lager_user_ids -> Text,
    }
}

diesel::table! {
    users (strapi_id) {
        strapi_id -> Integer,
        username -> Text,
        rolle -> Text,
    }
}

diesel::table! {
    einstellungen (id) {
        id -> Integer,
        barcode_mindestlaenge -> Integer,
        leitcodes_aktiv -> Bool,
        ausnahmen_aktiv -> Bool,
    }
}

diesel::table! {
    ausnahmen (id) {
        id -> Integer,
        barcode -> Text,
        bedeutung -> Text,
    }
}

diesel::table! {
    leitcodes (id) {
        id -> Integer,
        beschreibung -> Text,
        mindeslaenge -> Integer,
        leitcode_buchstabe -> Text,
        produktion -> Bool,
    }
}

