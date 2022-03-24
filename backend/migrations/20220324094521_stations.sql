-- Add migration script here
CREATE TABLE IF NOT EXISTS Stations (
    id SERIAL NOT NULL,
    opuic INTEGER NOT NULL,
    name VARCHAR(256) NOT NULL,
    abbrev VARCHAR(128),
    locality VARCHAR(128) NOT NULL,
    cantonName VARCHAR(128) NOT NULL,
    cantonAbbr VARCHAR(4) NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    long DOUBLE PRECISION NOT NULL,
    PRIMARY KEY(id)
);