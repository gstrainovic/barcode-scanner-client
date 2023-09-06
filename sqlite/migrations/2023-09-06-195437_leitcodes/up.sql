-- CREATE TABLE leitcode_buchstabe (
--     id INTEGER PRIMARY KEY NOT NULL,
--     buchstabe TEXT NOT NULL,
--     position INTEGER NOT NULL
-- );

CREATE TABLE leitcodes (
    id INTEGER PRIMARY KEY NOT NULL,
    beschreibung TEXT NOT NULL,
    mindeslaenge INTEGER NOT NULL,
    leitcode_buchstabe JSON NOT NULL
);