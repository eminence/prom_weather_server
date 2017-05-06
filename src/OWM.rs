//! Structs for the Open Weather Map API


use ::{UpdatableWeatherData, get_json_from_url};
use ::units::*;
use ::time;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OWMData {
    pub weather: Vec<WeatherData>,
    pub main: MainData,
    pub visibility: u32,
    pub wind: WindData,
    pub dt: u64,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct WeatherData {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct MainData {
    pub temp: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub temp_min: f32,
    pub temp_max: f32
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct WindData {
    pub speed: f32,
    pub deg: Option<u16>
}

impl UpdatableWeatherData for OWMData {
    fn update() -> Option<OWMData> {
        println!("{} Getting OWM weather data", time::now().rfc822());
        match get_json_from_url("http://api.openweathermap.org/data/2.5/weather?id=5225507&APPID=APIKEY") {
            Ok(data) => {
                // some sanity checks on this data.  If it doesn't look valid, don't return it
                Some(data)
            }
            Err(e) => {
                println!("Failed to fetch OWM weather data: {:?}", e.description);
                None
            }
        }
    }
    fn name(&self) -> &'static str { "owm" }
    fn get_temp(&self) -> Option<temperature::Celsius> {
        Some(temperature::Celsius(self.main.temp - 273.15))
    }
    fn get_pressure(&self) -> Option<pressure::Hectopascal> {
        Some(pressure::Hectopascal(self.main.pressure as f32))
    }
    fn get_humidity(&self) -> Option<humidity::Percent> {
        Some(humidity::Percent(self.main.humidity as u8))
    }
    fn get_wind_speed(&self) -> Option<speed::MetersPerSec> {
        Some(speed::MetersPerSec(self.wind.speed))
    }
    fn get_visibility(&self) -> Option<distance::Meters> {
        Some(distance::Meters(self.visibility))
    }
}
