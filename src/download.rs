extern crate reqwest;

use std::time::Duration;
// use serde_json::Value;
use crate::utils;

/// Returns our own Http Client with a 10 sec timeout
pub fn create_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder().timeout(Duration::from_secs(10)).build().ok()
}

/// Returns the default API call by city name with optional arguments concatenated into a String
fn concat_url(api_id: String,
    query: utils::Query,
    city_name: String,
    state_code: Option<String>,
    country_code: Option<String>,
    unit: utils::Temp) -> String
{
    // Patern matching the query type
    let query_type = match query
    {
        utils::Query::Weather => String::from("weather"), // Get current weather
        utils::Query::Forecast => String::from("forecast"), // Get forecast
    };

    // Patern matching the state code
    let state_str = match state_code
    {
        None => String::new(), // Get an empty string if there's None
        Some(s) => format!(",{}", s) // Prefix a comma if there's Some
    };

    // Patern matching the country code
    let country_str = match country_code
    {
        None => String::new(), // Get an empty string if there's None
        Some(s) => format!(",{}", s) // Prefix a comma if there's Some
    };

    let temp_unit = match unit
    {
        utils::Temp::K => String::new(),
        utils::Temp::C => String::from("&units=metric"),
        utils::Temp::F => String::from("&units=imperial"),
    };
    
    format!("http://api.openweathermap.org/data/2.5/{}?q={}{}{}&appid={}{}",
        &query_type,
        &city_name,
        &state_str,
        &country_str,
        &api_id,
        &temp_unit
    )
}

/// Performs the API call and returns the JSON
pub fn call_api(client: Option<reqwest::blocking::Client>,
    query_type: utils::Query,
    api_id: String,
    city_name: String,
    state_code: Option<String>,
    country_code: Option<String>,
    temp_unit: utils::Temp) -> Option<String>
{
    // Patern matching the client argument
    let my_client = match client
    {
        None => { // if it's None, create a new one
            match create_client()
            {
                Some(c) => c,
                None => panic!("Couldn't create Http Client"),
            }
        },
        Some(c) => c, // If it already has Some value, unwrap it
    };

    let url = concat_url(api_id, query_type, city_name, state_code, country_code, temp_unit);
    let url = reqwest::Url::parse(&url).unwrap();
    let req = my_client.get(url).send();

    // Patern matching the request result
    match req
    {
        Err(_)  => panic!("Couldn't get a response"),
        Ok(res) => {
            if let Ok(val) = res.text() // If there is a body
            {
                if !val.is_empty()
                {
                    Some(val)
                }
                else
                {
                    println!("Got an empty response body");
                    None
                }
            }
            else
            {
                println!("Got an empty/unparseable response");
                None
            }
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn creates_client() {
        assert!(super::create_client().is_some());
    }

    #[test]
    fn concats_urls() {
        let key = String::from("960c6bd08da0acfb2debc737573930c6");

        // Testing for weather query url with City name and temp in Celsius
        assert!(
            super::concat_url(
                key.clone(),
                super::utils::Query::Weather,
                String::from("City"),
                None,
                None,
                super::utils::Temp::C
            ) == format!("http://api.openweathermap.org/data/2.5/weather?q=City&appid={}&units=metric", key.clone())
        );

        // Testing for forecast query url with City name, State code and temp in Fahrenheit
        assert!(
            super::concat_url(
                key.clone(),
                super::utils::Query::Forecast,
                String::from("City"),
                Some(String::from("State")),
                None,
                super::utils::Temp::F
            ) == format!("http://api.openweathermap.org/data/2.5/forecast?q=City,State&appid={}&units=imperial", key.clone())
        );

        // Testing for worecast query url with City name, Country code and temp in Fahrenheit
        assert!(
            super::concat_url(
                key.clone(),
                super::utils::Query::Weather,
                String::from("City"),
                None,
                Some(String::from("Country")),
                super::utils::Temp::F
            ) == format!("http://api.openweathermap.org/data/2.5/weather?q=City,Country&appid={}&units=imperial", key.clone())
        );

        // Testing for worecast query url with City name, State code, Country code and temp in Celsius
        assert!(
            super::concat_url(
                key.clone(),
                super::utils::Query::Forecast,
                String::from("City"),
                Some(String::from("State")),
                Some(String::from("Country")),
                super::utils::Temp::C
            ) == format!("http://api.openweathermap.org/data/2.5/weather?q=City,State,Country&appid={}&units=metric", key.clone())
        );
    }
}
