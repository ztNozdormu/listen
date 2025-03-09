#!/bin/bash

sudo cp ./target/release/listen /usr/local/bin

sudo cp listener.service /etc/systemd/system/
sudo cp buyer.service /etc/systemd/system/

sudo chmod 644 /etc/systemd/system/listener.service
sudo chmod 644 /etc/systemd/system/buyer.service

sudo launchctl enable listener.service
sudo launchctl start listener.service

sudo launchctl enable buyer.service
sudo launchctl start buyer.service
