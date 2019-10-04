#!/bin/bash
echo Tobi Pizda
wget -q https://github.com/mshassium/rust_weather_app/raw/master/release/v1.0/rust-weather-cli
chmod u+x ./rust-weather-cli
mv ./rust-weather-cli /usr/local/bin/rweather
echo Запускаю епта.......
echo команда rweather
sleep 1s
rweather
