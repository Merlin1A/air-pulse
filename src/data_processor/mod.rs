mod database;
use crate::database;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AQIData {
    pub pm25: i32,
    pub o3: i32,
    pub pm10: i32,
}

#[derive(Debug)]
pub struct ForecastAQIData {
    pub pm25: i32,
    pub o3: i32,
    pub pm10: i32,
}

pub fn process_aqi_data(user_id: i32, aqi_data: &HashMap<String, AQIData>) -> Result<(), Box<dyn std::error::Error>> {
    let user_prefs = database::load_user_preferences(user_id)?;

    for (location, data) in aqi_data {
        if data.pm25 > user_prefs.pm25_threshold {
            // tmp
        }
        if data.o3 > user_prefs.o3_threshold {
            // tmp
        }
        if data.pm10 > user_prefs.pm10_threshold {
            // tmp
        }
    }
    Ok(())
}

pub fn calculate_forecast_accuracy(forecast_data: &HashMap<String, ForecastAQIData>, actual_data: &HashMap<String, AQIData>) -> HashMap<String, f64> {
    let mut accuracy_map: HashMap<String, f64> = HashMap::new();

    for (location, forecast) in forecast_data {
        if let Some(actual) = actual_data.get(location) {
            let pm25_accuracy = ((forecast.pm25 - actual.pm25).abs() as f64) / forecast.pm25 as f64;
            let o3_accuracy = ((forecast.o3 - actual.o3).abs() as f64) / forecast.o3 as f64;
            let pm10_accuracy = ((forecast.pm10 - actual.pm10).abs() as f64) / forecast.pm10 as f64;

            let overall_accuracy = (pm25_accuracy + o3_accuracy + pm10_accuracy) / 3.0;
            accuracy_map.insert(location.clone(), overall_accuracy);
        }
    }

    accuracy_map
}

