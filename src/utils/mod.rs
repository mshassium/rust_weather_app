const APPID : &str = "566e3e4fa2e15f8874dd5a8afb8af4d4";
const CURRENT_WEATHER_URL : &str = "https://api.openweathermap.org/data/2.5/weather?q={cityKey}&APPID={appidKey}";
const DAYTIME_WEATHER_URL : &str = "https://api.openweathermap.org/data/2.5/forecast?q={cityKey}&APPID={appidKey},&cnt={dayCount}";

pub fn current_weather_url(city : &str) -> String {
    let result_url: String = CURRENT_WEATHER_URL.replace("{cityKey}",city).replace("{appidKey}",APPID);
    return result_url;
}

pub fn daytime_weather_url(city: &str, count_day: u32) -> String {
    let result_url: String = DAYTIME_WEATHER_URL
                             .replace("{cityKey}",city)
                             .replace("{appidKey}",APPID)
                             .replace("{dayCount}",&count_day.to_string());
    return result_url;
}