extern crate reqwest;
use std::io;
use std::collections::HashMap;
use std::collections::LinkedList;
use serde::{Deserialize};

#[derive(Deserialize)]
struct WeatherResponse {
    coord: HashMap<String,f32>,
    weather: LinkedList<Weather>,
    base: String,
    main: HashMap<String,f32>,
    wind: HashMap<String,f32>,
    clouds: HashMap<String,f32>,
    dt: u32,
    sys: Sys,
    timezone: u32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String
}

#[derive(Deserialize)]
struct Sys {
    r#type: f32,
    id: f32,
    message: f32,
    country: String,
    sunrise: f32,
    sunset: f32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let appid = "566e3e4fa2e15f8874dd5a8afb8af4d4";
    let url_raw = "https://api.openweathermap.org/data/2.5/weather?q={cityKey}&APPID={appidKey}";
    let kelvin_value = 273.15;
    println!("Hello, please write city full name");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Not readable");
    println!("You city name is: {}\nPlease wait....",line);
    let result_url = url_raw.replace("{cityKey}",&line).replace("{appidKey}",appid);
    let resp: WeatherResponse = reqwest::get(&result_url)?.json()?;
    let correctTemp = resp.main.get("temp").unwrap() - kelvin_value;
    println!("Correct temp is {:.2}", correctTemp);
    Ok(())
}
