-- Add migration script here
CREATE TABLE IF NOT EXISTS occupancy (
    connectionDate DATE NOT NULL,
    trainNr INTEGER NOT NULL,
    bikes INTEGER[] NOT NULL,
    PRIMARY KEY(connectionDate, trainNr)    
);