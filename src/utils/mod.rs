pub fn weather_url(city : String) -> String {
    let appid = "566e3e4fa2e15f8874dd5a8afb8af4d4";
    let url_raw = "https://api.openweathermap.org/data/2.5/weather?q={cityKey}&APPID={appidKey}";
    let result_url: String = url_raw.replace("{cityKey}",&city).replace("{appidKey}",appid);
    return result_url;
}