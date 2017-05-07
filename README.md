# prom_weather_server

Fetches basic weather info (temperaure, pressure, windspeed, visibility, humidity) from
[Open Weather Map](https://openweathermap.org/), [Weather Underground](https://www.wunderground.com/),
and the [National Weather Service](https://forecast-v3.weather.gov/documentation).

The data from each service is refreshed every 10 minutes, and is exported via a web server so that
it can be imported into [Prometheus](https://prometheus.io/)
