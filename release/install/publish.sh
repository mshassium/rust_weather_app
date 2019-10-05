#!/bin/bash
echo Publish start
cargo build
mv ../../target/debug/rweather ./rweather
git add ../../.
git commit -m "publish new build"
git push origin master
echo Publish finish