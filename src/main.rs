#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod utils;
mod download;

#[get("/weather?<city>&<state>&<country>&<unit>")]
fn weather(city: String,
    state: Option<String>,
    country: Option<String>,
    unit: Option<String>) -> String
{
    let temp_unit = match unit {
        Some(s) if s == "c" || s == "C" => utils::Temp::C,
        Some(s) if s == "f" || s == "F" => utils::Temp::C,
        _ => utils::Temp::K,
    };
    download::call_api(None,
        utils::Query::Weather,
        String::from("960c6bd08da0acfb2debc737573930c6"),
        city,
        state,
        country,
        temp_unit
    ).unwrap()
}

#[get("/forecast?<city>&<state>&<country>&<unit>")]
fn forecast(city: String,
    state: Option<String>,
    country: Option<String>,
    unit: Option<String>) -> String
{
    let temp_unit = match unit {
        Some(s) if s == "c" || s == "C" => utils::Temp::C,
        Some(s) if s == "f" || s == "F" => utils::Temp::C,
        _ => utils::Temp::K,
    };
    download::call_api(None,
        utils::Query::Forecast,
        String::from("960c6bd08da0acfb2debc737573930c6"),
        city,
        state,
        country,
        temp_unit
    ).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![weather, forecast])
        .launch();
    
}
