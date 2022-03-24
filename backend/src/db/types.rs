use chrono::{Date, DateTime, NaiveTime, Utc};

struct Weather {
    id: i32,
    station_id: String,
    date: DateTime<Utc>,
    biking: f32,
    snow_depth: f32,
    temperature: f32,
    rainfall: f32,
    weather: i32,
    cloud_cover: i32,
}

struct Holidays {
    id: i32,
    canton: String,
    population: i64,
    spring_start: Date<Utc>,
    spring_end: Date<Utc>,
    summer_start: Date<Utc>,
    summer_end: Date<Utc>,
    fall_start: Date<Utc>,
    fall_end: Date<Utc>,
}

struct DataEntry {
    id: i32,
    reservationDate: Date<Utc>,
    reservationTime: Option<NaiveTime>,
    train_number: i32,
    train_line: String,
    reserved: i32,
    capacity: Option<i32>,
    station_from: String,
    station_to: String,
    departure: DateTime<Utc>,
    arrival: DateTime<Utc>,
}
