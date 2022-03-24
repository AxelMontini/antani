use reqwest::Url;
use rocket::local::asynchronous::LocalRequest;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "http://transport.opendata.ch/v1";
const LOCATION: &str = "http://transport.opendata.ch/v1/location";
const CONNECTIONS: &str = "http://transport.opendata.ch/v1/connections";
const STATIONBOARD: &str = "http://transport.opendata.ch/v1/stationboard";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LocationType {
    Station,
    Poi,
    Address,
    Refine,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    id: u64,
    #[serde(rename = "type")]
    ty: LocationType,
    name: String,
    score: 
}

#[derive(Debug, Serialize, Deserialize)]
struct LocationResponse {
    stations: Vec<Location>
}

#[derive(Serialize, Deserialize, Debug)]
struct LocationRequest {
    /// Location to search for
    query: String,
    x: f32,
    y: f32,
    #[serde(rename = "type")]
    ty: LocationRequestType,
}

impl Default for LocationRequest {
    fn default() -> Self {
        Self {
            query: "".into(),
            x: 0.0,
            y: 0.0,
            ty: LocationRequestType::All,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LocationRequestType {
    All,
    Station,
    Poi,
    Address,
}
