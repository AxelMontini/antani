-- Add migration script here
CREATE TABLE IF NOT EXISTS Dataset (
    id SERIAL NOT NULL,
    reservationDate DATE NOT NULL,
    reservationTime TIME WITHOUT TIME ZONE,
    connectionDate DATE NOT NULL,
    trainNr INTEGER NOT NULL,
    line VARCHAR(128) NOT NULL,
    reserved INTEGER NOT NULL,
    capacity INTEGER,
    stationFrom VARCHAR(8) NOT NULL,
    stationTo VARCHAR(8) NOT NULL,
    depTimestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    arrTimestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    PRIMARY KEY(id)
);