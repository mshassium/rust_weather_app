#!/bin/bash
echo Tobi Pizda
wget -q https://github.com/mshassium/rust_weather_app/raw/master/release/v1.0/rweather
chmod u+x ./rweather
mv ./rweather /usr/local/bin/rweather
echo Запускаю епта.......
echo команда rweather
sleep 1s
rweather