#!/bin/bash

# cargo does not allow to specify per-package targets, and i cannot set a thumbv7-none-eabi target for the whole workspace
# without that, cargo does not know that it should use probe-rs runner
# so i use this shell script to build, flash and attach via probe-rs

cargo run --package vegvisir-controller --target thumbv7em-none-eabi