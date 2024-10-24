-- Add migration script here
CREATE TABLE IF NOT EXISTS family (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    family_id INTEGER,
    FOREIGN KEY(family_id) REFERENCES family(id)
);
