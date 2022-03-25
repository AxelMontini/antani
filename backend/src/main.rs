use std::{ops::Deref, sync::Arc};

use anyhow::Context;
use api::{
    Connection, ConnectionRequest, ConnectionResponse, LocationRequest, CONNECTIONS, LOCATIONS,
};
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, Utc};
use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use rocket::{
    form::FromFormField,
    get,
    response::status,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    Either, State,
};

use sqlx::{postgres::PgPoolOptions, PgPool};
use structopt::StructOpt;

use crate::api::LocationRequestType;

mod api;
mod db;
mod utils;

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
pub async fn stops(
    client: &State<Client>,
    db: &State<PgPool>,
    trainNr: i32,
) -> Result<Json<Stops>, status::BadRequest<()>> {
    let stops = get_stops(client, db, trainNr)
        .await
        .map_err(|_| status::BadRequest(None))?;
    Ok(Json(Stops { stops }))
}

pub async fn get_stops(client: &Client, _db: &PgPool, trainNr: i32) -> anyhow::Result<Vec<String>> {
    let params = [
        ("dataset", "ist-daten-sbb"),
        ("q", &trainNr.to_string()),
        ("sort", "ab_prognose"),
    ];

    Ok(client
        .get("https://data.sbb.ch/api/records/1.0/search/")
        .query(&params)
        .send()
        .await?
        .json::<Response>()
        .await?
        .get_stations())
}

/// get abbreviation of <station>: "Zurich HB" -> "ZUE"
#[get("/abbrev?<station>")]
async fn abbrev(
    db: &State<PgPool>,
    station: &str,
) -> Result<String, status::BadRequest<&'static str>> {
    get_abbrev(db, &station)
        .await
        .map_err(|_e| status::BadRequest(Some("database error")))?
        .ok_or(status::BadRequest(Some("station does not exist")))
}

async fn get_abbrev(db: &PgPool, station: &str) -> anyhow::Result<Option<String>> {
    let query = sqlx::query!(r#"SELECT abbrev FROM stations WHERE name=$1"#, station);
    let response = query
        .fetch_one(db)
        .await
        .context("fetch abbrev of station")?;
    Ok(response.abbrev)
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
) -> Result<Json<Option<Capacity>>, status::BadRequest<String>> {
    // TODO: Check if date should be used as Swiss Timezone or Utc in the database
    let date = date
        .parse::<NaiveDate>()
        .map_err(|e| status::BadRequest(Some(e.to_string())))?;

    let cap = get_capacity(db, date, trainNr)
        .await
        .map_err(|e| status::BadRequest(Some(e.to_string())))?;

    Ok(Json(cap))
}

async fn get_capacity(
    db: &PgPool,
    date: NaiveDate,
    trainNr: i32,
) -> anyhow::Result<Option<Capacity>> {
    let query = sqlx::query!(
        r#"SELECT max(capacity) FROM dataset WHERE connectionDate=$1 and trainNr=$2"#,
        date,
        trainNr
    );
    let response = query
        .fetch_one(db.deref())
        .await
        .context("fetching train capacity")?;

    Ok(response.max.map(|max| Capacity {
        date,
        trainNr,
        capacity: max,
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
    _db: &State<PgPool>,
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
    //let text = res.text().await.unwrap();
    let cr: ConnectionResponse = res.json().await.unwrap();
    //println!("JSON: {}", text);
    //let cr: ConnectionResponse = serde_json::from_str(&text).unwrap();

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

#[derive(Serialize)]
struct Holiday {
    isHoliday: bool,
}

#[get("/holidays?<name>&<date>")]
async fn holidays(
    db: &State<PgPool>,
    name: String,
    date: String,
) -> Result<Json<Holiday>, status::BadRequest<String>> {
    let date = date
        .parse::<NaiveDate>()
        .map_err(|e| status::BadRequest(Some(e.to_string())))?;

    let response = sqlx::query!(
        "SELECT count(*) FROM ((SELECT cantonName FROM stations WHERE name=$1) INTERSECT (SELECT canton as cantonName FROM schoolholidays WHERE (fallStart <= $2 and fallEnd >= $2) or (summerStart <= $2 and summerEnd >= $2) or (springStart <= $2 and springEnd >= $2))) as bbb",
        name,
        date
    ).fetch_one(db.deref()).await.unwrap();
    let flag = response.count.unwrap_or(0);
    Ok(Json(Holiday {
        isHoliday: flag == 1,
    }))
}

struct Token {
    val: String,
    expiration: DateTime<Utc>,
}

impl Token {
    /// Returns true whether the token is very close to expiration or has already expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration + Duration::seconds(50)
    }
}

#[derive(derive_more::Deref)]
struct ClientSecret(Arc<str>);

#[derive(derive_more::Deref)]
struct ClientId(Arc<str>);

#[derive(Deserialize, Debug)]
struct TokenObject {
    access_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
}

struct TokenState(rocket::tokio::sync::RwLock<Token>);

impl Deref for TokenState {
    type Target = rocket::tokio::sync::RwLock<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Returns a token or not lmao
async fn auth(client: &Client, client_id: &str, client_secret: &str) -> anyhow::Result<Token> {
    let token_url = "https://sso.sbb.ch/auth/realms/SBB_Public/protocol/openid-connect/token";

    let token_resp = client
        .post(token_url)
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await?;

    let token_obj: TokenObject = token_resp.json().await?;

    let token = Token {
        val: token_obj.access_token,
        expiration: Utc::now() + Duration::seconds(token_obj.expires_in),
    };

    Ok(token)
}

/// get weather information for the <date> at the train <station>: date=2022-10-22T16:16:16Z&station=Chiasso -> [temperature, rainfall, weather]
#[get("/weather?<date>&<station>")]
async fn weather(
    client: &State<Client>,
    db: &State<PgPool>,
    client_secret: &State<ClientSecret>,
    client_id: &State<ClientId>,
    token_state: &State<TokenState>,
    date: String,
    station: String,
) -> Json<MeteoData> {
    let token = token_state.read().await;

    let token = if token.is_expired() {
        // MUST drop token before modifying it, otherwise we end up causing a deadlock
        std::mem::drop(token);
        {
            // Authenticate and set new token in RwLock
            let token = auth(client, &client_id, &client_secret)
                .await
                .expect("major fuckup happened when authenticating to the weather API"); // TODO: Can handle? Probably not gracefully
            let mut token_write = token_state.write().await;
            *token_write = token;
        }
        token_state.read().await
    } else {
        token
    };

    let url = "https://weather.api.sbb.ch:443";
    let params = "t_2m:C,precip_1h:mm,weather_symbol_1h:idx";
    let coord = coordinates(db, station).await.0;

    let req = format!(
        "{}/{}/{}/{},{}/json",
        url, date, params, coord.lat, coord.long
    );

    let meteo_resp = client.post(req).bearer_auth(&token.val).send().await.unwrap();
    let raw_array: serde_json::Value = meteo_resp.json().await.unwrap();
    let mut acc = vec![];
    let j = raw_array.get("data").unwrap();
    for i in 0..3 {
        let data = j.get(i).unwrap();
        let param = data.get("parameter").unwrap().as_str().unwrap();
        let value = data // TODO: handle errors like a human being
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
        acc.push(Data {
            parameter: param.to_string(),
            value,
        });
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

async fn fill_occupancy(client: &Client, db: &PgPool) -> anyhow::Result<()> {
    let all_travels = sqlx::query!(
        "(SELECT connectionDate, trainNr FROM Dataset GROUP BY connectionDate, trainNr) EXCEPT (SELECT connectionDate, trainNr FROM Occupancy)"
    );
    all_travels
        .fetch_many(db)
        .map(|r| r.map_err(|e| anyhow::anyhow!(e)))
        .try_for_each(|e| async {
            println!("FOR EACH");
            match e {
                Either::Right(rec) => {
                    let trainNr = rec.trainnr.context("No train nr")?;
                    let date = rec.connectiondate.context("No connection date")?;

                    let bikes = occupancy(client, db, trainNr, date).await?;
                    println!(
                        ">> TrainNr: {}, date: {}, Bikes: {:?}",
                        trainNr, date, bikes
                    );
                    Ok(())
                }
                o => Err(anyhow::anyhow!("What is going on here {:?}", o)),
            }
        })
        .await?;

    Ok(())
}

async fn occupancy(
    client: &Client,
    db: &PgPool,
    trainNr: i32,
    date: NaiveDate,
) -> anyhow::Result<Vec<i32>> {
    let stops = get_stops(client, db, trainNr).await?;
    println!("JOIN ALL");
    let abbrevs = futures::future::join_all(stops.iter().map(|e| get_abbrev(db, e))).await;
    // let capacity = get_capacity(db, date, trainNr)
    //     .await?
    //     .map(|c| c.capacity)
    //     .unwrap_or(0);

    let legsAmount = stops.len(); // was - 1
    let mut legs = vec![0; legsAmount];

    let postgresDate = date;
    println!("Select");
    let query = sqlx::query!("SELECT stationFrom, stationTo, reserved FROM dataset WHERE connectionDate=$1 and trainNr=$2", postgresDate, trainNr);
    let response = query.fetch_all(db.deref()).await.unwrap();

    response.iter().for_each(|rec| {
        let mut flag = false;
        for (index, s) in abbrevs.iter().enumerate() {
            match s {
                Ok(Some(s)) => {
                    flag |= s == &rec.stationfrom;
                    if flag {
                        if s.clone() == rec.stationto {
                            break;
                        }
                        legs[index] += rec.reserved;
                    }
                }
                o => panic!("Wrong thing {:?}", o), //TODO: Do something when this happens
            }
        }
    });
    legs.pop();
    Ok(legs)
}

#[derive(StructOpt)]
#[structopt(about = "SBB Bikes server and cli", name = "sbb-bikes")]
struct Opts {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    /// Start the web server
    Start(StartCmd),
    /// Generate occupancy data
    Gen(GenCmd),
}

#[derive(StructOpt)]
struct GenCmd {
    #[structopt(short, long, env)]
    database_url: String,
}

#[derive(StructOpt)]
struct StartCmd {
    #[structopt(short, long, env)]
    database_url: String,
    #[structopt(long, env)]
    weather_id: String,
    #[structopt(long, env)]
    weather_secret: String,
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    match opts.command {
        Subcommand::Start(s) => start(s).await,
        Subcommand::Gen(g) => gen(g).await,
    }
}

async fn gen(opts: GenCmd) -> anyhow::Result<()> {
    let db_uri = opts.database_url;

    let client = reqwest::Client::new();
    let pool = PgPoolOptions::new().connect(&db_uri).await?;

    fill_occupancy(&client, &pool).await
}

async fn start(opts: StartCmd) -> anyhow::Result<()> {
    let db_uri = opts.database_url;

    let client = reqwest::Client::new();
    let pool = PgPoolOptions::new().connect(&db_uri).await?;

    rocket::build()
        .manage(client)
        .manage(pool)
        .manage(ClientSecret(opts.weather_secret.into())) // secret and id used for weather
        .manage(ClientId(opts.weather_id.into()))
        .manage(TokenState(rocket::tokio::sync::RwLock::new(Token {
            // invalid token since the start, auth lazily
            val: "".into(),
            expiration: Utc::now() - Duration::seconds(60),
        })))
        .mount(
            "/",
            routes![
                locations,
                stations,
                stops,
                abbrev,
                capacity,
                connections,
                coordinates,
                weather,
                holidays
            ],
        )
        .launch()
        .await
        .context("rocket error")
}
