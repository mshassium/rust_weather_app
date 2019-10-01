extern crate reqwest;
extern crate rust_weather_cli;
use rust_weather_cli::utils;
use serde_json;
use std::io;
use whoami;

const KELVIN: f64 = 273.15;

enum AppMode {
    CurrentWeather,
    DayTimeWeather,
    Exit,
    Wrong,
}

fn main() {
    println!(
        "Hello {}, please chose what do you want.",
        whoami::username()
    );
    loop {
        println!("****************************************");
        println!("1 = Write city name and see current weather");
        println!("2 = Write city name, day count and see weather for this period");
        println!("3 = Exit :)");
        println!("****************************************");
        let mut request_mod_string: String = String::new();
        let mut app_mod: AppMode = AppMode::Exit;
        match io::stdin().read_line(&mut request_mod_string) {
            Ok(n) => {
                app_mod = match request_mod_string.as_str().trim() {
                    "1" => AppMode::CurrentWeather,
                    "2" => AppMode::DayTimeWeather,
                    "3" => AppMode::Exit,
                    _ => AppMode::Wrong,
                };
            }
            Err(error) => println!("Error: {}", error),
        }
        let app_result_message: String = match app_mod {
            AppMode::CurrentWeather => current_weather(),
            AppMode::DayTimeWeather => String::from("Day Time Weather"),
            AppMode::Exit => {
                println!("Bye");
                break;
            }
            AppMode::Wrong => String::from("Error"),
        };
        println!("{}", app_result_message);
    }
}

fn current_weather() -> String {
    let mut city_name: String = String::new();
    println!("Please write city name");
    match io::stdin().read_line(&mut city_name) {
        Err(error) => println!("{}", error),
        _ => print!("{}", ""),
    }
    let resp: String = reqwest::get(&utils::current_weather_url(&city_name))
        .unwrap()
        .text()
        .unwrap();
    let resp_value: serde_json::value::Value = serde_json::from_str(&resp).unwrap();
    let cod = resp_value.get("cod").unwrap();
    let final_temp_string = if cod == 200 {
        format!(
            "{}°C in {}",
            resp_value
                .get("main")
                .and_then(|value| value.get("temp"))
                .and_then(|value| value.as_f64())
                .and_then(|value| Some((value - KELVIN).round()))
                .and_then(|value| Some(value.to_string()))
                .unwrap(),
            &city_name
        )
    } else {
        format!(
            "Error: {}",
            resp_value
                .get("message")
                .and_then(|value| Some(value.to_string()))
                .unwrap()
        )
    };
    final_temp_string
}
