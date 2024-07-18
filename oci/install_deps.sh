#!/bin/sh
set -e
apt-get update
apt-get install -y libssl3 ca-certificates
rm -rf /var/lib/apt/lists/*
