#!/bin/bash
echo Publish start
cargo build
mv ../../target/debug/rust-weather-cli ./rweather
git add ../../.
git commit -m "publish new build"
git push origin master
echo Publish finish