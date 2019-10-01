extern crate reqwest;
extern crate rust_weather_cli;
use std::io;
use rust_weather_cli::utils;
use serde_json;
use whoami;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello {}, please chose what do you want?",whoami::username());
    let kelvin_value = 273.15;
    println!("Hello, please write city full name");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Not readable");
    println!("You city name is: {}\nPlease wait....",line);
    let resp: String = reqwest::get(&utils::current_weather_url(line))?.text()?;
    let resp_value: serde_json::value::Value = serde_json::from_str(&resp)?;
    let cod = resp_value.get("cod").unwrap();
    let final_temp_string = 
        if cod == 200{
              format!("Correct temp is {}°C",resp_value.get("main")
                        .and_then(|value| value.get("temp"))
                        .and_then(|value| value.as_f64())
                        .and_then(|value| Some((value - kelvin_value).round()))
                        .and_then(|value| Some(value.to_string())).unwrap())
                    }
        else {
            format!("Error: {}",resp_value.get("message").and_then(|value| Some(value.to_string())).unwrap())
        };
    println!("{}", final_temp_string);
    Ok(())
}
