#!/usr/bin/env bash
set -e

nice cargo build --release

# systemd
sudo mkdir -p /usr/local/lib/systemd/system/
sudo cp -uv ./*.service /usr/local/lib/systemd/system/
sudo systemctl daemon-reload

# stop, replace and start new version
sudo systemctl stop mpd-internetradio-destuck.service
sudo cp -v target/release/mpd-internetradio-destuck /usr/local/bin
sudo systemctl enable --now mpd-internetradio-destuck.service
