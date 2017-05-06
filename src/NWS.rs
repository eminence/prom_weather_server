//! Structs for the National Weather Service API

use ::{UpdatableWeatherData, get_json_from_url};
use ::units::*;
use ::time;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct NWSData {
    pub properties: NWSDataInner
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct NWSDataInner {
    pub temperature: NWSProperty,
    pub windSpeed: NWSProperty,
    pub windDirection: NWSProperty,
    pub barometricPressure: NWSProperty,
    pub visibility: NWSProperty,
    pub relativeHumidity: NWSProperty,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct NWSProperty {
    pub value: Option<f32>,
    pub unitCode: String,
    pub qualityControl: String
}

impl UpdatableWeatherData for NWSData {
    fn update() -> Option<NWSData> {
        println!("{} Getting NWS weather data", time::now().rfc822());
        let r: Result<NWSData, _> =  get_json_from_url("https://api.weather.gov/stations/KPVD/observations/current");
        match r {
            Ok(data) => {
                // confirm that we have valid temperature data
                let has_temp = data.properties.temperature.value.is_some();
                if has_temp {
                    Some(data)
                } else {
                    println!("Missing temperature data from NWS");
                    None
                }
            }
            Err(e) => {
                println!("Failed to fetch NWS weather data: {:?}", e.description);
                None
            }
        }
    }
    fn name(&self) -> &'static str { "nws" }
    fn get_temp(&self) -> Option<temperature::Celsius> {
        self.properties.temperature.value.map(|t| temperature::Celsius(t))
    }
    
    fn get_pressure(&self) -> Option<pressure::Hectopascal> {
        self.properties.barometricPressure.value.map(|t| pressure::Hectopascal(t / 100.0))
    }
    fn get_humidity(&self) -> Option<humidity::Percent> {
        self.properties.relativeHumidity.value.map(|t| humidity::Percent(t.round() as u8))
    }
    fn get_wind_speed(&self) -> Option<speed::MetersPerSec> {
        self.properties.windSpeed.value.map(|t| speed::MetersPerSec(t))
    }
    fn get_visibility(&self) -> Option<distance::Meters> {
        self.properties.visibility.value.map(|t| distance::Meters(t.round() as u32))
    }
}
