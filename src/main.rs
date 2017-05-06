extern crate reqwest;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate time;

use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::Mutex;
use std::convert::From;
use std::error::Error;
use hyper::server::{Server, Request, Response};
use hyper::header::ContentLength;
use std::io::Write;

mod OWM;
use OWM::*;

mod NWS;
use NWS::*;

mod WUG;
use WUG::*;

pub mod units;

pub struct WeatherLoadingError {
    pub description: String
}

impl From<reqwest::Error> for WeatherLoadingError {
    fn from(e: reqwest::Error) -> WeatherLoadingError {
        WeatherLoadingError { description: e.description().to_owned() }
    }
}

trait UpdatableWeatherData: Sized {
    /// Load weather data from 
    fn update() -> Option<Self>;
    fn name(&self) -> &'static str;
    fn get_temp(&self) -> Option<units::temperature::Celsius>;
    fn get_pressure(&self) -> Option<units::pressure::Hectopascal>;
    fn get_humidity(&self) -> Option<units::humidity::Percent>;
    fn get_wind_speed(&self) -> Option<units::speed::MetersPerSec>;
    fn get_visibility(&self) -> Option<units::distance::Meters>;
}


struct CachedData<T> {
    latest_data: T,
    refresh_interval: Duration,
    updated: Instant
}


type CachedOWMData = CachedData<OWMData>;
type CachedNWSData = CachedData<NWSData>;
type CachedWUGData = CachedData<WUGData>;


impl<T: UpdatableWeatherData> CachedData<T> {
    fn new(offset: u64) -> CachedData<T> {
        CachedData {
            latest_data: T::update().unwrap(),
            updated: Instant::now() - Duration::from_secs(offset*60),
            refresh_interval: Duration::from_secs(10*60)
        }
    }

    fn update(&mut self) {
        if self.updated.elapsed() > self.refresh_interval {
            if let Some(data) = T::update() {
                self.latest_data = data;
                self.updated = Instant::now();
            } else {
                println!("Failed to get some data.  Will try again next time");
            }
        }
    }
}





pub fn get_json_from_url<U: reqwest::IntoUrl, T: serde::de::Deserialize>(url: U) -> Result<T, WeatherLoadingError> {
        let client = reqwest::Client::new()?;
        let mut resp = client.get(url).send()?;
        Ok(resp.json()?)
}



struct MyHandler {
    owmdata: Arc<Mutex<CachedOWMData>>,
    nwsdata: Arc<Mutex<CachedNWSData>>,
    wugdata: Arc<Mutex<CachedWUGData>>,

}

impl MyHandler {
    fn new() -> MyHandler {

        MyHandler {
            owmdata: Arc::new(Mutex::new(CachedOWMData::new(0))),
            nwsdata: Arc::new(Mutex::new(CachedNWSData::new(3))),
            wugdata: Arc::new(Mutex::new(CachedWUGData::new(6)))
        }
    }
}

fn write_data<T: UpdatableWeatherData>(body: &mut String, guard: &T) {

    let name = guard.name();

    if let Some(data) = guard.get_temp() {
        body.push_str(&format!("outside_temp{{cityid=\"5225507\",source=\"{}\"}} {}\n", name, data.0));
    }
    if let Some(data) = guard.get_pressure() {
        body.push_str(&format!("outside_pressure{{cityid=\"5225507\",source=\"{}\"}} {}\n", name, data.0));
    }

    if let Some(data) = guard.get_humidity() {
        body.push_str(&format!("outside_humidity{{cityid=\"5225507\",source=\"{}\"}} {}\n", name, data.0));
    }

    if let Some(data) = guard.get_wind_speed() {
        body.push_str(&format!("outside_wind_speed{{cityid=\"5225507\",source=\"{}\"}} {}\n", name, data.0));
    }

    if let Some(data) = guard.get_visibility() {
        body.push_str(&format!("outside_visibility{{cityid=\"5225507\",source=\"{}\"}} {}\n", name, data.0));
    }
}

impl hyper::server::Handler for MyHandler {


    fn handle(&self, req: Request, mut res: Response) {
        let mut body = String::new();


        if let Ok(mut guard) = self.owmdata.lock() {
            guard.update();
            write_data(&mut body, &guard.latest_data);
        }
        if let Ok(mut guard) = self.nwsdata.lock() {
            guard.update();
            write_data(&mut body, &guard.latest_data);
        }
        if let Ok(mut guard) = self.wugdata.lock() {
            guard.update();
            write_data(&mut body, &guard.latest_data);
        }


        res.headers_mut().set(ContentLength(body.len() as u64));
        let mut res = res.start().unwrap();
        res.write_all(body.as_bytes()).unwrap();

    }
}

fn main() {

    Server::http("127.0.0.1:9102").unwrap().handle(MyHandler::new()).unwrap();
}
