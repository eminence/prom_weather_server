//! Structs for the National Weather Service API

use ::{UpdatableWeatherData, get_json_from_url};
use ::units::*;
use std::str::FromStr;
use ::time;

#[derive(Debug, Deserialize)]
pub struct WUGData {
    current_observation: WUGObservations
}

#[derive(Debug, Deserialize)]
pub struct WUGObservations {
    temp_c: f32,
    relative_humidity: String,
    wind_degrees: u16,
    wind_kph: f32,
    pressure_mb: String,
    visibility_km: String

}

impl UpdatableWeatherData for WUGData {
    fn update() -> Option<WUGData> {
        println!("{} Getting WUG weather data", time::now().rfc822());
        match get_json_from_url("http://api.wunderground.com/api/APIKEY/conditions/q/RI/Warwick.json") {
            Ok(data) => {
                Some(data)
            }
            Err(e) => {
                println!("Failed to fetch WUG weather data: {:?}", e.description);
                None
            }
        }
    }
    fn name(&self) -> &'static str { "wug" }
    fn get_temp(&self) -> Option<temperature::Celsius> {
        Some(temperature::Celsius(self.current_observation.temp_c))
    }
    
    fn get_pressure(&self) -> Option<pressure::Hectopascal> {
        f32::from_str(&self.current_observation.pressure_mb).ok().map(|v| pressure::Hectopascal(v))
    }
    fn get_humidity(&self) -> Option<humidity::Percent> {
        u8::from_str(self.current_observation.relative_humidity.trim_matches('%')).ok().map(|v| humidity::Percent(v))
    }
    fn get_wind_speed(&self) -> Option<speed::MetersPerSec> {
        Some(speed::MetersPerSec(self.current_observation.wind_kph * 1000.0 / 3600.0))
    }
    fn get_visibility(&self) -> Option<distance::Meters> {
        f32::from_str(&self.current_observation.visibility_km).ok().map(|v| distance::Meters((v * 1000.0) as u32))
    }
}
