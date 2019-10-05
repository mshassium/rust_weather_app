extern crate reqwest;
extern crate rweather;
#[macro_use]
extern crate clap;

use rweather::utils;
use serde_json;
use clap::App;

const KELVIN: f64 = 273.15;

fn main() {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        let city: String = matches.value_of("city").unwrap_or("Default city 😁").to_string();
        let day_count: u32 = matches.value_of("day_count").unwrap_or("0").parse().unwrap();
        let result : String = match day_count {
            0 | 1 => current_weather(city),
            _ => day_time_weather()
        };
        println!("{}", result);
    }

fn day_time_weather() -> String{
    String::from("😁 Ну типа я работаю, но как бы нет 😁")
}

fn current_weather(city_name: String) -> String {
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
