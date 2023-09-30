extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};
use std::env;

/// Represents the color associated with a particular Air Quality Index (AQI).
#[derive(Serialize, Deserialize, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// Represents an Air Quality Index (AQI) with its associated information.
#[derive(Serialize, Deserialize, Debug)]
struct Index {
    code: String,
    displayName: String,
    aqi: i32,
    aqiDisplay: String,
    color: Color,
    category: String,
    dominantPollutant: String,
}

/// Represents the concentration of a particular pollutant.
#[derive(Serialize, Deserialize, Debug)]
struct Concentration {
    value: f64,
    units: String,
}

/// Represents additional information about a particular pollutant.
#[derive(Serialize, Deserialize, Debug)]
struct AdditionalInfo {
    sources: String,
    effects: String,
}

/// Represents a pollutant with its associated information.
#[derive(Serialize, Deserialize, Debug)]
struct Pollutant {
    code: String,
    displayName: String,
    fullName: String,
    concentration: Concentration,
    additionalInfo: AdditionalInfo,
}

/// Represents health recommendations based on the current air quality.
#[derive(Serialize, Deserialize, Debug)]
struct HealthRecommendations {
    generalPopulation: String,
    elderly: String,
}

/// Represents the air quality data retrieved from the Google Air Quality API.
#[derive(Serialize, Deserialize, Debug)]
pub struct AirQualityData {
    dateTime: String,
    regionCode: String,
    indexes: Vec<Index>,
    pollutants: Vec<Pollutant>,
    healthRecommendations: HealthRecommendations,
}

/// Represents the request body to be sent to the Google Air Quality API.
#[derive(Serialize, Debug)]
struct RequestBody {
    universal_aqi: bool,
    location: Location,
    extra_computations: Vec<String>,
    language_code: String,
}

/// Represents the geographic location for which air quality data is to be retrieved.
#[derive(Serialize, Debug)]
struct Location {
    latitude: f64,
    longitude: f64,
}

/// Provides functionality to interact with the Google Air Quality API.
pub struct DataFetcher {
    client: Client,
    api_key: String,
}

impl DataFetcher {
    pub fn new() -> Self {
        let client = Client::new();
        let api_key = env::var("GOOGLE_AIR_QUALITY_API_KEY")
            .expect("Google Air Quality API Key could not be retrieved.");
        Self { client, api_key }
    }

    /// Fetches the current air quality data for the specified geographic location.
    ///
    /// # Arguments
    ///
    /// * `latitude` - The latitude of the location.
    /// * `longitude` - The longitude of the location.
    ///
    /// # Returns
    ///
    /// * `Ok(AirQualityData)` if the request is successful.
    /// * `Err(Error)` if the request fails.
    pub fn fetch_air_quality_data(&self, latitude: f64, longitude: f64) -> Result<AirQualityData, Error> {
        let url = format!(
            "https://airquality.googleapis.com/v1/currentConditions:lookup?key={}",
            self.api_key
        );

        let body = RequestBody {
            universal_aqi: true,
            location: Location { latitude, longitude },
            extra_computations: vec![
                "HEALTH_RECOMMENDATIONS".to_string(),
                "DOMINANT_POLLUTANT_CONCENTRATION".to_string(),
                "POLLUTANT_CONCENTRATION".to_string(),
                "LOCAL_AQI".to_string(),
                "POLLUTANT_ADDITIONAL_INFO".to_string(),
            ],
            language_code: "en".to_string(),
        };

        let response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?;

        if response.status().is_success() {
            response.json::<AirQualityData>()
        } else {
            Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "Failed to retrieve air quality data",
            ))
        }
    }
}