use schemars::JsonSchema;
use chrono::{DateTime, FixedOffset};


use serde::{Deserialize, Serialize};

const PREFIX: &str = "http://transport.opendata.ch/v1";
pub const LOCATIONS: &str = "http://transport.opendata.ch/v1/locations";
pub const CONNECTIONS: &str = "http://transport.opendata.ch/v1/connections";
pub const STATIONBOARD: &str = "http://transport.opendata.ch/v1/stationboard";


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: Option<LocationType>,
    pub name: String,
    pub score: Option<f32>,
    pub coordinate: Coordinate,
    pub distance: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Coordinate {
    #[serde(rename = "type")]
    pub ty: String,
    /// latitude
    pub x: f32,
    /// longitude
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub stations: Vec<Location>,
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
    pub from: String,
    pub to: String,
    pub via: Option<Vec<String>>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub is_arrival_time: Option<bool>,
    #[serde(rename = "transportations[]")]
    pub transportations: Option<String>,
    /// 1..=16
    pub limit: Option<u8>,
    /// 0..=3
    pub page: Option<u8>,
    pub direct: Option<bool>,
    pub sleeper: Option<bool>,
    pub couchette: Option<bool>,
    pub bike: Option<bool>,
    //pub accessibility: Option<bool>,
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

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ConnectionResponse {
    pub connections: Vec<Connection>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Connection {
    pub from: Checkpoint,
    pub to: Checkpoint,
    pub duration: String,
    pub service: Option<Service>,
    pub products: Vec<String>,
    pub capacity1st: Option<u8>,
    pub capacity2nd: Option<u8>,
    pub sections: Vec<Section>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Section {
    pub journey: Option<Journey>,
    pub walk: Option<f32>,
    pub departure: Checkpoint,
    pub arrival: Checkpoint,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Journey {
    /// Equivalent to train number in the db
    pub name: String,
    pub category: String,
    pub categoryCode: Option<u32>,
    pub number: String,
    pub operator: String,
    pub to: String,
    pub passList: Vec<Checkpoint>,
    pub capacity1st: Option<u8>,
    pub capacity2nd: Option<u8>,
}

/// What is this for???
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Service {
    pub regular: String,
    pub irregular: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Checkpoint {
    pub station: Location,
    pub arrival: Option<DateTime<FixedOffset>>,
    pub departure: Option<DateTime<FixedOffset>>,
    pub delay: Option<u32>,
    pub platform: Option<String>,
    pub prognosis: Option<Prognosis>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Prognosis {
    pub arrival: Option<DateTime<FixedOffset>>,
    pub departure: Option<DateTime<FixedOffset>>,
    pub platform: Option<String>,
    pub capacity1st: Option<String>,
    pub capacity2nd: Option<String>,
}

#[cfg(test)]
mod tests {
    use rocket::tokio;
    use schemars::schema_for;

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

        let _cr: ConnectionResponse = resp.json().await.unwrap();
    }

    #[test]
    fn schemas() {
        let schema = schema_for!(ConnectionResponse);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
