-- Your SQL goes here
CREATE TABLE IF NOT EXISTS history
(
    id INTEGER PRIMARY KEY NOT NULL,
    status TEXT NOT NULL,
    barcode TEXT NOT NULL,
    timestamp TEXT NOT NULL
);