-- Add migration script here
CREATE TABLE IF NOT EXISTS SchoolHolidays (
    id SERIAL NOT NULL,
    canton VARCHAR(128) NOT NULL,
    population INTEGER NOT NULL,
    springStart DATE NOT NULL,
    springEnd DATE NOT NULL,
    summerStart DATE NOT NULL,
    summerEnd DATE NOT NULL,
    fallStart DATE NOT NULL,
    fallEnd DATE NOT NULL,
    PRIMARY KEY(id)
);