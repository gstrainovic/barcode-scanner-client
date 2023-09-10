CREATE TABLE IF NOT EXISTS leitcodes (
    id INTEGER PRIMARY KEY NOT NULL,
    beschreibung TEXT NOT NULL,
    mindeslaenge INTEGER NOT NULL,
    leitcode_buchstabe JSON NOT NULL
);