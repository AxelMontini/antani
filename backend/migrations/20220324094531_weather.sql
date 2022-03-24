-- Add migration script here
CREATE TABLE weather (
    id SERIAL NOT NULL,
    stationId VARCHAR(25) NOT NULL,
    leisureBiking DOUBLE PRECISION NOT NULL,
    snowDepth DOUBLE PRECISION NOT NULL,
    twom DOUBLE PRECISION NOT NULL,
    precip DOUBLE PRECISION NOT NULL,
    weatherSymbol INTEGER NOT NULL,
    cloudCover INTEGER NOT NULL,
    date DATE,
    PRIMARY KEY(id)
);