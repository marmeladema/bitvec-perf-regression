#!/bin/bash

set -euxo

cargo build --release
exec setarch x86_64 -R nice -20 taskset -c 3,7 cargo run --release -- results.json
