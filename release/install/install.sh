#!/bin/bash
echo Start installation
wget https://github.com/mshassium/rust_weather_app/raw/master/release/v.0.0.1/rweather
chmod u+x ./rweather
mv ./rweather /usr/local/bin/rweather
echo Installation finish.......
sleep 2s
rweather -c Saratov