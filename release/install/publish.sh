#!/bin/bash
echo 📖 Publish start
cargo build
mv ../../target/debug/rweather ../v.0.0.1/rweather
git add ../../.
git commit -m "publish new build"
git push origin master
echo 📖 Publish finish