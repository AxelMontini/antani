use std::{ops::Deref, str::FromStr};

use anyhow::Context;
use api::{
    Connection, ConnectionRequest, ConnectionResponse, LocationRequest, CONNECTIONS, LOCATIONS,
};
use chrono::{DateTime, NaiveDate, Utc, FixedOffset};
use reqwest::Client;
use rocket::{
    form::FromFormField,
    get, launch,
    response::status,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::api::LocationRequestType;

mod api;
mod db;

#[derive(Serialize, Debug)]
struct Locations {}

#[get("/locations?<query>&<ty>")]
async fn locations(
    client: &State<Client>,
    query: &str,
    ty: Option<&str>,
) -> Result<Json<Locations>, status::BadRequest<String>> {
    let ty = ty
        .map(|ty| {
            let t = LocationRequestType::try_from(ty);
            match t {
                Ok(t) => Ok(t),
                Err(e) => Err(status::BadRequest(Some(format!(
                    "ty is not acceptable: {}",
                    e
                )))),
            }
        })
        .transpose();

    let ty = match ty {
        Ok(ty) => ty,
        Err(e) => return Err(e),
    };

    let req = LocationRequest {
        query: Some(query.into()),
        ty,
        ..Default::default()
    };

    client.get(LOCATIONS).query(&req);

    Ok(Json(Locations {}))
}

#[derive(Serialize)]
struct Stations {
    stations: Vec<String>,
}

/// get all the station names starting with <start>: "Chi" -> ["Chiasso", ...]
#[get("/stations?<start>")]
async fn stations(db: &State<PgPool>, start: String) -> Json<Stations> {
    let query = sqlx::query!(
        r#"SELECT name FROM stations WHERE lower(name) LIKE lower($1 || '%')"#,
        start
    );

    let response = query.fetch_all(db.deref()).await.unwrap(); //TODO: Handle error and do not PANIK!!
    println!("{:?}", response);
    let dio = Stations {
        stations: response.iter().map(|e| return e.name.to_string()).collect(),
    };

    Json(dio)
}

#[derive(Deserialize, Debug)]
struct Response {
    records: Vec<Record>,
}

#[derive(Serialize)]
struct Stops {
    stops: Vec<String>,
}

impl Response {
    pub fn get_stations(self) -> Vec<String> {
        self.records
            .iter()
            .map(|e| e.fields.haltestellen_name.clone())
            .collect()
    }
}

#[derive(Deserialize, Debug)]
struct Record {
    pub fields: Fields,
}

#[derive(Deserialize, Debug)]
struct Fields {
    pub haltestellen_name: String,
}

/// get all the stops of train number <trainNr>: 324 -> ["Chiasso", ...]
#[get("/stops?<trainNr>")]
async fn stops(client: &State<Client>, db: &State<PgPool>, trainNr: i32) -> Json<Stops> {
    let params = [
        ("dataset", "ist-daten-sbb"),
        ("q", &trainNr.to_string()),
        ("sort", "ab_prognose"),
    ];
    let res = client
        .get("https://data.sbb.ch/api/records/1.0/search/")
        .query(&params)
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap()
        .get_stations();
    Json(Stops { stops: res })
}

/// get abbreviation of <station>: "Zurich HB" -> "ZUE"
#[get("/abbrev?<station>")]
async fn abbrev(db: &State<PgPool>, station: String) -> String {
    let query = sqlx::query!(
        r#"SELECT abbrev FROM stations WHERE name=$1 or locality=$1"#,
        station
    );
    let response = query.fetch_one(db.deref()).await.unwrap();
    response.abbrev.unwrap()
}

#[derive(Serialize)]
struct Capacity {
    date: NaiveDate,
    trainNr: i32,
    capacity: i32,
}

/// Takes a date in format YYYY-mm-dd
#[get("/capacity?<date>&<trainNr>")]
async fn capacity(
    db: &State<PgPool>,
    date: String,
    trainNr: i32,
) -> Result<Json<Capacity>, status::BadRequest<String>> {
    // TODO: Check if date should be used as Swiss Timezone or Utc in the database
    let date = date
        .parse::<DateTime<Utc>>()
        .map_err(|e| status::BadRequest(Some(e.to_string())))?
        .date()
        .naive_local();

    let query = sqlx::query!(
        r#"SELECT max(capacity) FROM dataset WHERE connectionDate=$1 and trainNr=$2"#,
        date,
        trainNr
    );
    let response = query.fetch_one(db.deref()).await.unwrap();
    Ok(Json(Capacity {
        date,
        trainNr,
        capacity: response.max.unwrap(),
    }))
}

#[derive(derive_more::From, derive_more::Into, Serialize, Deserialize)]
struct DateTimeUtc(DateTime<Utc>);

impl Deref for DateTimeUtc {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'f> FromFormField<'f> for DateTimeUtc {}

trait ToStatus {
    fn st(&self) -> status::BadRequest<String>;
}

impl ToStatus for anyhow::Error {
    fn st(&self) -> status::BadRequest<String> {
        status::BadRequest(Some(self.to_string()))
    }
}

#[derive(Serialize)]
struct FConnections {
    connections: Vec<FConnection>,
}

#[derive(Serialize)]
struct FConnection {
    score: f32,
    connection: Connection,
}

// Get connections for the given addresses
#[get("/connections?<from>&<to>&<datetime>&<is_arrival_time>")]
async fn connections(
    db: &State<PgPool>,
    client: &State<Client>,
    from: String,
    to: String,
    datetime: String,
    is_arrival_time: bool,
) -> Result<Json<FConnections>, status::BadRequest<String>> {
    // We assume that bike=true since it's the website for bicycles
    let datetime = DateTime::<FixedOffset>::parse_from_rfc3339(datetime.as_str())
        .map_err(|e| e.to_string())
        .map_err(Some)
        .map_err(status::BadRequest)?;
    let date = Some(datetime.format("%Y-%m-%d").to_string());
    let time = Some(datetime.format("%H-%M-%S").to_string());
    let req = ConnectionRequest {
        from,
        to,
        date,
        time,
        is_arrival_time: Some(is_arrival_time),
        bike: Some(true),
        ..Default::default()
    };

    let res = client.get(CONNECTIONS).query(&req).send().await.unwrap();
    let cr: ConnectionResponse = res.json().await.unwrap();

    // Try to sort connections based on bike places availability
    //let algorithm = |a| 1.0;

    Ok(Json(FConnections {
        connections: cr
            .connections
            .into_iter()
            .take(5)
            .map(|c| FConnection {
                score: 1.0,
                connection: c,
            })
            .collect(),
    }))
}

#[derive(Serialize)]
struct Coordinates {
    lat: f64,
    long: f64,
}

/// get the coordinates of the <station>: Chiasso -> {15.0, 45.2}
#[get("/coordinates?<station>")]
async fn coordinates(db: &State<PgPool>, station: String) -> Json<Coordinates> {
    let query = sqlx::query!("SELECT lat, long FROM stations WHERE name=$1", station);
    let response = query.fetch_one(db.deref()).await.unwrap();
    Json(Coordinates {
        lat: response.lat,
        long: response.long,
    })
}

/// get weather information for the <date> at the train <station>: date=2022-10-22T16:16:16Z&station=Chiasso -> [temperature, rainfall, weather]
#[get("/weather?<date>&<station>")]
async fn weather(client: &State<Client>, db: &State<PgPool>, date: String, station: String) -> Json<MeteoData> {
    let token = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJOMDhQek52bDdqNGFfSlBmZ0FlZFNYTHNjcmprbmZ4OXppR2hxcHN1dkt3In0.eyJleHAiOjE2NDgxMzY0NDEsImlhdCI6MTY0ODEzMTk0MSwianRpIjoiOTcwMzE0OTktNzUzOS00ZDVlLTkyZWYtODU5MjllODgwOTJlIiwiaXNzIjoiaHR0cHM6Ly9zc28uc2JiLmNoL2F1dGgvcmVhbG1zL1NCQl9QdWJsaWMiLCJhdWQiOiJhcGltLXdlYXRoZXJfc2VydmljZS1wcm9kLWF3cyIsInN1YiI6IjQ1NGNiMjM5LTZjN2EtNGU3Zi05YzY3LWQ0ZWQyMDAzMzdmYyIsInR5cCI6IkJlYXJlciIsImF6cCI6Ijg1OGY2MzRmIiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwczovL2RldmVsb3Blci5zYmIuY2giXSwic2NvcGUiOiJjbGllbnQtaW5mbyBzYmJ1aWQgcHJvZmlsZSBlbWFpbCBTQkIiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImNsaWVudEhvc3QiOiIyMTcuMTkyLjEwMi4xNCIsImNsaWVudElkIjoiODU4ZjYzNGYiLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJzZXJ2aWNlLWFjY291bnQtODU4ZjYzNGYiLCJjbGllbnRBZGRyZXNzIjoiMjE3LjE5Mi4xMDIuMTQiLCJlbWFpbCI6InNlcnZpY2UtYWNjb3VudC04NThmNjM0ZkBwbGFjZWhvbGRlci5vcmcifQ.k2dTEXqjHTeInatjFF-L0Y9aOwQpehBMdKB9LQB-6zVuT4KQGvJVKcxqGF-zydbiWoMJ4Mi1jG3VhCOzNiSaARJ1Cq7577eonfq9zQBUfiag8EOLvBsj4lZ-vk2kJhuHBzoxx-XO0DhxtIKEZE715pw5OR8j2gh1EJCbXfniV8vVtOR7e5ND1yGCujREYgqf5g90152ewKPN5bDtdVukbo-Jj4qD0aRc5z1oxhoc56oj2eJmtqrzkAT4Tx3Kdu7Nu46RgGyXCEblMEePeWm5FXUNbGKCVisrTNOI5OMKi_NplYqoc7g6d7az9t1ZuwBup30xolR85C_WYPJb9epA2Q";
    let url = "https://weather.api.sbb.ch:443";
    let params = "t_2m:C,precip_1h:mm,weather_symbol_1h:idx";
    let coord = coordinates(db, station).await.0;

    let req = format!(
        "{}/{}/{}/{},{}/json",
        url, date, params, coord.lat, coord.long
    );

    let a = client.post(req).bearer_auth(token).send().await.unwrap();
    let raw_array: serde_json::Value = a.json().await.unwrap();
    let mut acc = vec![];
    let j = raw_array.get("data").unwrap();
    for i in 0..3 {
        let data = j.get(i).unwrap();
        let param = data.get("parameter").unwrap().as_str().unwrap();
        let value = data
            .get("coordinates")
            .unwrap()
            .get(0)
            .unwrap()
            .get("dates")
            .unwrap()
            .get(0)
            .unwrap()
            .get("value")
            .unwrap()
            .as_f64()
            .unwrap();
        acc.push(Data{parameter: param.to_string(), value});
    }
    Json(MeteoData { data: acc })
}

#[derive(Serialize)]
struct MeteoData {
    data: Vec<Data>,
}

#[derive(Serialize)]
struct Data {
    parameter: String,
    value: f64,
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let pool = PgPoolOptions::new()
        .connect("postgres://chad:thundercockftw@lanciapini.axelmontini.dev:32100/postgresdb")
        .await?;

    rocket::build()
        .manage(client)
        .manage(pool)
        .mount("/", routes![locations, stations, stops, abbrev, capacity, connections, coordinates, weather])
        .launch()
        .await
        .context("rocket error")
}
