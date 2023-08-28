use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    Number: i32,
    Name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    DateIssue: String,
    DateForecast: String,
    ReportingArea: String,
    StateCode: String,
    Latitude: f64,
    Longitude: f64,
    ParameterName: String,
    AQI: i32,
    Category: Category,
    ActionDay: bool,
    Discussion: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentObservation {
    DateObserved: String,
    HourObserved: i32,
    LocalTimeZone: String,
    ReportingArea: String,
    StateCode: String,
    Latitude: f64,
    Longitude: f64,
    ParameterName: String,
    AQI: i32,
    Category: Category,
}

pub struct DataFetcher {
    api_key: String,
}

impl DataFetcher {
    pub fn new() -> Self {
        let api_key = env::var("AIRNOW_API_KEY").expect("AIRNOW API Key could not be retrieved.");
        Self { api_key }
    }

    pub fn fetch_forecast_by_zip(&self, zip_code: &str, date: &str, distance: i32) -> Result<Vec<Forecast>, Error> {
        let url = format!(
            "https://www.airnowapi.org/aq/forecast/zipCode/?format=application/json&zipCode={}&date={}&distance={}&API_KEY={}",
            zip_code,
            date,
            distance,
            self.api_key
        );

        let response = reqwest::blocking::get(&url)?;

        let forecasts = if response.status().is_success() {
            response.json::<Vec<Forecast>>()?
        } else {
            vec![]
        };

        Ok(forecasts)
    }

    pub fn fetch_current_observation_by_zip(&self, zip_code: &str, distance: i32) -> Result<Vec<CurrentObservation>, Error> {
        let url = format!(
            "https://www.airnowapi.org/aq/observation/zipCode/current/?format=application/json&zipCode={}&distance={}&API_KEY={}",
            zip_code,
            distance,
            self.api_key
        );

        let response = reqwest::blocking::get(&url)?;

        let observations = if response.status().is_success() {
            response.json::<Vec<CurrentObservation>>()?
        } else {
            vec![]
        };

        Ok(observations)
    }
}
