use chrono::{Date, DateTime, Duration, DurationRound, NaiveTime, Utc};
use reqwest::Url;
use rocket::local::asynchronous::LocalRequest;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "http://transport.opendata.ch/v1";
pub const LOCATIONS: &str = "http://transport.opendata.ch/v1/locations";
pub const CONNECTIONS: &str = "http://transport.opendata.ch/v1/connections";
pub const STATIONBOARD: &str = "http://transport.opendata.ch/v1/stationboard";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Station,
    Poi,
    Address,
    Refine,
}

impl TryFrom<&str> for LocationType {
    type Error = LocationTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "station" => Ok(Self::Station),
            "poi" => Ok(Self::Poi),
            "refine" => Ok(Self::Refine),
            "address" => Ok(Self::Address),
            o => Err(LocationTypeError(o.into())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid LocationTypeError: \"{0}\"")]
pub struct LocationTypeError(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    id: String,
    #[serde(rename = "type")]
    ty: Option<LocationType>,
    name: String,
    score: Option<f32>,
    coordinate: Coordinate,
    distance: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    #[serde(rename = "type")]
    ty: String,
    /// latitude
    x: f32,
    /// longitude
    y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    stations: Vec<Location>,
}

#[derive(Debug, Default, Serialize)]
pub struct LocationRequest {
    /// Location to search for
    pub query: Option<String>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    #[serde(rename = "type")]
    pub ty: Option<LocationRequestType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationRequestType {
    All,
    Station,
    Poi,
    Address,
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid LocationRequestTypeError: \"{0}\"")]
pub struct LocationRequestTypeError(String);

impl TryFrom<&str> for LocationRequestType {
    type Error = LocationRequestTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all" => Ok(Self::All),
            "station" => Ok(Self::Station),
            "poi" => Ok(Self::Poi),
            "address" => Ok(Self::Address),
            e => Err(LocationRequestTypeError(e.into())),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ConnectionRequest {
    from: String,
    to: String,
    via: Option<Vec<String>>,
    date: Option<String>,
    time: Option<String>,
    is_arrival_time: Option<bool>,
    #[serde(rename = "transportations[]")]
    transportations: Option<String>,
    /// 1..=16
    limit: Option<u8>,
    /// 0..=3
    page: Option<u8>,
    direct: Option<bool>,
    sleeper: Option<bool>,
    couchette: Option<bool>,
    bike: Option<bool>,
    //accessibility: Option<bool>,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(into = "String")]
pub enum TransportationType {
    Train,
    Tram,
    Ship,
    Bus,
    Cableway,
}

impl Into<String> for TransportationType {
    fn into(self) -> String {
        match self {
            TransportationType::Train => "train",
            TransportationType::Tram => "tram",
            TransportationType::Ship => "ship",
            TransportationType::Bus => "bus",
            TransportationType::Cableway => "cableway",
        }
        .into()
    }
}

#[derive(Deserialize, Debug)]
pub struct ConnectionResponse {
    connections: Vec<Connection>,
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    from: Checkpoint,
    to: Checkpoint,
    duration: String,
    service: Service,
    products: Vec<String>,
    capacity1st: u8,
    capacity2nd: u8,
    sections: Vec<Section>,
}

#[derive(Deserialize, Debug)]
pub struct Section {
    journey: Option<Journey>,
    walk: Option<f32>,
    departure: Checkpoint,
    arrival: Checkpoint,
}

#[derive(Deserialize, Debug)]
pub struct Journey {
    name: String,
    category: String,
    categoryCode: u32,
    number: u32,
    operator: u32,
    to: String,
    passList: Vec<Checkpoint>,
    capacity1st: u8,
    capacity2nd: u8,
}

/// What is this for???
#[derive(Deserialize, Debug)]
pub struct Service {
    regular: String,
    irregular: String,
}

#[derive(Deserialize, Debug)]
pub struct Checkpoint {
    station: Location,
    arrival: NaiveTime,
    departure: Option<NaiveTime>,
    delay: Option<u32>,
    platform: u32, // TODO: Can be string?
    prognosis: Option<Prognosis>,
}

#[derive(Deserialize, Debug)]
pub struct Prognosis {
    arrival: DateTime<Utc>,
    departure: Option<DateTime<Utc>>,
    platform: u32,
    capacity1st: u8,
    capacity2nd: u8,
}

#[cfg(test)]
mod tests {
    use rocket::tokio;

    use super::*;

    #[tokio::test]
    async fn send_location() {
        let req = LocationRequest {
            query: Some("Taverne".into()),
            ..Default::default()
        };

        let client = reqwest::Client::new();

        let req = client.get(LOCATIONS).query(&req);

        let resp = req.send().await.unwrap();

        let lr: LocationResponse = resp.json().await.unwrap();

        println!("{:?}", lr);
    }

    #[tokio::test]
    async fn send_connection() {
        let client = reqwest::Client::new();

        let req = ConnectionRequest {
            from: "Taverne".into(),
            to: "Rivera".into(),
            date: Some("2021-05-22".into()),
            time: Some("11:00".into()),
            is_arrival_time: Some(true),
            transportations: Some("train".into()),
            ..Default::default()
        };

        let resp = client.get(CONNECTIONS).query(&req).send().await.unwrap();

        assert!(resp.status().is_success());

        let cr: ConnectionResponse = resp.json().await.unwrap();
    }
}
