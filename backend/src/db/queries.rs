use chrono::{Date, Utc};

fn isVacation(canton: String, date: Date<Utc>) -> bool {
    let iso_date = date.format("%Y-%m-%d").to_string();
    let res = sqlx::query!("select count(*) from (select 1 from schoolholidays where canton='$1' and fallStart <= '$2' and fallEnd >= '$2' and summerStart <= '$2' and summerEnd >= '$2' and fallStart <= '$2' and fallEnd >= '$2') as vacation", canton, iso_date)
        .fetch_one(&mut conn)
        .await
        .unwrap();
    res as bool
}

fn connect_to_db() {}


// posto -> coordinate
// nome: $1

// select lat, long from stations where name=$1 or locality=$1;

// get stops
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Response {
    records: Vec<Record>,
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

async fn getStops(trainNumber: i32) -> anyhow::Result<Vec<String>> {
    let params = [
        ("dataset", "ist-daten-sbb"),
        ("q", trainNumber.to_string()),
        ("sort", "ab_prognose"),
    ];
    let client = reqwest::Client::new();
    let res: Response = client
        .get("https://data.sbb.ch/api/records/1.0/search/")
        .query(&params)
        .send()
        .await?
        .json::<Response>()
        .await?;
}