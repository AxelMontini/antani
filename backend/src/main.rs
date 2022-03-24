use std::ops::Deref;

use anyhow::Context;
use api::{LocationRequest, LOCATIONS};
use reqwest::Client;
use rocket::{get, launch, response::status, routes, serde::{Serialize, json::Json}, State};
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

#[get("/stations/<start>")]
async fn stations(db: &State<PgPool>, start: String) -> Json<Stations> {
    let query = sqlx::query!(
        r#"SELECT name FROM stations WHERE lower(name) LIKE lower($1 || '%')"#,
        start
    );

    let response = query.fetch_all(db.deref()).await.unwrap();
    println!("{:?}", response);
    let dio = Stations {
        stations: response.iter().map(|e| return e.name.to_string()).collect(),
    };
    Json(dio)
}

// Get connections for the given addresses
// #[get("/connections?<from>&<to>&<datetime>&<is_arrival>")]
// async fn connections(from: String, to: String, dateime: String, is_arrival: bool) -> () {

// }

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let pool = PgPoolOptions::new()
        .connect("postgres://chad:thundercockftw@lanciapini.axelmontini.dev:32100/postgresdb")
        .await?;

    rocket::build()
        .manage(client)
        .manage(pool)
        .mount("/", routes![locations, stations])
        .launch()
        .await
        .context("rocket error")
}
