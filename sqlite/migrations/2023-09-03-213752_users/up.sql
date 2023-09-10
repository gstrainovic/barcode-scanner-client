-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    strapi_id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    rolle TEXT NOT NULL
);
