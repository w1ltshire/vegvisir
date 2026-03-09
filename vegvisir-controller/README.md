# `vegvisir-controller`
<p>
<img alt="trans rights" src="https://pride-badges.pony.workers.dev/static/v1?label=trans%20rights&stripeWidth=6&stripeColors=5BCEFA,F5A9B8,FFFFFF,F5A9B8,5BCEFA">
</p>

---

## Overview
This crate contains the firmware that runs on the microcontroller of the drone. As of now, it targets only
`stm32f401re` board.

## Building
`vegvisir-controller` can be built as usual with `cargo build`

## Running
Due to Cargo's limitations on `.cargo/config.toml` in workspaces, to run this project via `probe-rs`
run `./build_and_flash.sh` script, which will execute `cargo run` with proper target flag (thumbv7em-none-eabi).

## License
[MIT](https://choosealicense.com/licenses/mit/)