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
        Some(s) if s == "f" || s == "F" => utils::Temp::F,
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
        Some(s) if s == "f" || s == "F" => utils::Temp::F,
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

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;

    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn gets_weather() {
        let my_rocket = rocket::ignite().mount("/", routes![super::weather, super::forecast]);
        let client = Client::new(my_rocket).expect("valid rocket instance");

        let mut response = client.get("/weather?city=London&unit=C").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_matches!(response.body_string(), Some(_));
    }

    #[test]
    fn gets_forecast() {
        let my_rocket = rocket::ignite().mount("/", routes![super::weather, super::forecast]);
        let client = Client::new(my_rocket).expect("valid rocket instance");

        let mut response = client.get("/weather?city=Barcelona,VE&unit=F").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_matches!(response.body_string(), Some(_));
    }

    #[test]
    fn fails_weather() {
        let my_rocket = rocket::ignite().mount("/", routes![super::weather, super::forecast]);
        let client = Client::new(my_rocket).expect("valid rocket instance");

        let mut response = client.get("/weather?city=London,VE&unit=F").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(String::from("{\"cod\":\"404\",\"message\":\"city not found\"}")));
    }
}
