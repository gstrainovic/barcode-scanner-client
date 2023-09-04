// @generated automatically by Diesel CLI.

diesel::table! {
    history (id) {
        id -> Integer,
        status -> Text,
        barcode -> Text,
        timestamp -> Text,
        synced -> Bool,
        user_id -> Integer,
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



