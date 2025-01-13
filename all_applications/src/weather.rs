use clap::Parser;
use serde::Deserialize;
use std::error::Error;

#[derive(Parser)]
#[command(name = "Weather CLI")]
#[command(about = "Fetches and displays weather information for a given city.")]
struct Cli {
    /// The city to fetch weather information for.
    #[arg(short, long)]
    city: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

const API_KEY: &str = "c09fd12e890863cd96b341ed1564ef2e"; //API key
const BASE_URL: &str = "http://api.openweathermap.org/data/2.5/weather";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let city = cli.city;

    match fetch_weather(&city).await {
        Ok(response) => display_weather(response),
        Err(err) => eprintln!("Error fetching weather data: {}", err),
    }

    Ok(())
}

async fn fetch_weather(city: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "{}?q={}&appid={}&units=metric",
        BASE_URL, city, API_KEY
    );

    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch weather data. HTTP Status: {}",
            response.status()
        )
        .into());
    }

    let weather_data = response.json::<WeatherResponse>().await?;
    Ok(weather_data)
}

fn display_weather(weather: WeatherResponse) {
    println!("Weather in {}:", weather.name);
    println!("Temperature: {:.2}°C", weather.main.temp);
    println!("Humidity: {}%", weather.main.humidity);

    if let Some(condition) = weather.weather.first() {
        println!("Condition: {}", condition.description);
    }
}
