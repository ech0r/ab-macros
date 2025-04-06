#!/usr/bin/env bash

set -e 

cargo check

if [[ "$1" == "release" ]]; then
	rm -rf release
	mkdir release
	trunk build --release -d release
else
	trunk serve --open
fi
