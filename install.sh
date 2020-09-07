#!/bin/sh

nice cargo build --release
sudo cp -uv target/release/mpd-internetradio-destuck /usr/local/bin/

# copy stuff
sudo cp -uv *.service /etc/systemd/system

# reload systemd
sudo systemctl daemon-reload
