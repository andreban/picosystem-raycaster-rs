# PicoSystem Raycaster

A Wolfenstein-style raycaster demo written in Rust for the [Pimoroni PicoSystem](https://shop.pimoroni.com/products/picosystem) handheld gaming device, powered by the RP2040 microcontroller.

## Overview

This project implements a basic 3D raycasting engine (similar to Wolfenstein 3D) running on embedded hardware with no operating system (`no_std`). It renders a first-person perspective view of a simple maze on the PicoSystem's 240×240 ST7789 display.

## Controls

| Button | Action        |
|--------|---------------|
| Up     | Move forward  |
| Down   | Move backward |
| Left   | Turn left     |
| Right  | Turn right    |

## Building

This project requires the Rust embedded toolchain targeting the RP2040.

1. Install the target:
   ```sh
   rustup target add thumbv6m-none-eabi
   ```

2. Install `elf2uf2-rs` or `probe-rs` for flashing:
   ```sh
   cargo install elf2uf2-rs
   ```

3. Build:
   ```sh
   cargo build --release
   ```

4. Flash to the PicoSystem by holding the X button while connecting via USB (puts it in BOOTSEL mode), then copy the `.uf2` file to the mounted drive, or use a debug probe.

## Project Structure

```
src/
├── main.rs           # Entry point, game loop, input handling
├── picosystem.rs     # PicoSystem hardware abstraction (buttons, display init)
├── st7789.rs         # ST7789 display driver
└── raycaster/
    ├── mod.rs        # Raycasting engine
    ├── hit.rs        # Ray hit detection types
    ├── map.rs        # Tile map definition
    └── player.rs     # Player state
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
