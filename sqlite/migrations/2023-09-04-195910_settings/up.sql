-- Your SQL goes here
CREATE TABLE einstellungen (
    id INTEGER PRIMARY KEY NOT NULL,
    barcode_mindestlaenge INTEGER NOT NULL,
    leitcodes_aktiv BOOLEAN DEFAULT false NOT NULL;
    ausnahmen_aktiv BOOLEAN DEFAULT false NOT NULL;
);

