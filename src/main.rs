extern crate reqwest;
extern crate rust_weather_cli;
use std::io;
use rust_weather_cli::utils;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kelvin_value = 273.15;
    println!("Hello, please write city full name");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Not readable");
    println!("You city name is: {}\nPlease wait....",line);
    let resp: String = reqwest::get(&utils::weather_url(line))?.text()?;
    let resp_value: serde_json::value::Value = serde_json::from_str(&resp)?;
    let final_temp_string = resp_value.get("main")
                            .and_then(|value| value.get("temp"))
                            .and_then(|value| value.as_f64())
                            .and_then(|value| Some(value - kelvin_value));
    println!("Correct temp is {:.0}°C", final_temp_string.unwrap());
    Ok(())
}
