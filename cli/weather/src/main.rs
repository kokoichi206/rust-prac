use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    lat: &str,
    lng: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    // https://openweathermap.org/current
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lng, api_key,
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    let description: &str = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_txt: String = format!(
        "Weather: in {}: {} {},
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored: ColoredString = match description {
        "clear sky" => weather_txt.green(),
        "few clouds" => weather_txt.yellow(),
        "scattered clouds" => weather_txt.yellow(),
        "broken clouds" => weather_txt.yellow(),
        "overcast clouds" => weather_txt.yellow(),
        "shower rain" => weather_txt.blue(),
        "rain" => weather_txt.blue(),
        "thunderstorm" => weather_txt.red(),
        "snow" => weather_txt.white(),
        "mist" => weather_txt.white(),
        _ => weather_txt.normal(),
    };
    println!("{}", weather_text_colored);

    fn get_temp_emoji(temperature: f64) -> &'static str {
        // ; => statement, it does not return a value
        let emoji = match temperature {
            t if t < 0.0 => "❄️",
            t if t < 15.0 => "🥶",
            t if t < 25.0 => "😌",
            _ => "🔥",
        };

        // expression that returns a value
        emoji
    }
}

fn main() {
    println!("{}", "hehehehehe".bright_yellow());

    loop {
        println!("{}", "plz enter the latitude");
        let mut lat = String::new();
        io::stdin()
            .read_line(&mut lat)
            .expect("failed to read input.");
        let lat: &str = lat.trim();

        println!("{}", "plz enter the longitude");
        let mut lon = String::new();
        io::stdin()
            .read_line(&mut lon)
            .expect("failed to read input.");
        let lon: &str = lon.trim();

        // see: https://home.openweathermap.org/api_keys
        let api_key = "YOUR_API_KEY";

        match get_weather_info(lat, lon, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
