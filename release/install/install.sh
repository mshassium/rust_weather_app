#!/bin/bash
echo Tobi Pizda
wget -q https://github.com/mshassium/rust_weather_app/raw/master/release/v.0.0.1/rweather
chmod u+x ./rweather
mv ./rweather /usr/local/bin/rweather
echo Запускаю епта.......
echo команда rweather
sleep 1s
rweather