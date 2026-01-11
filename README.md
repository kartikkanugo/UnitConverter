# UnitConverter

Rust based CLI UnitConverter

`uconv` is a simple command-line unit conversion tool written in Rust.  
It focuses on basic compressible-flow–related calculations involving **Mach number**, **velocity**, and **dynamic pressure**, using standard atmospheric assumptions.

The application uses the [`clap`](https://crates.io/crates/clap) crate for command-line argument parsing and exposes a small set of subcommands for different calculation entry points.

---

## Features

- Convert **Mach number** to:
  - Velocity (m/s)
  - Dynamic pressure (Pa)
- Convert **velocity (m/s)** to:
  - Mach number
  - Dynamic pressure (Pa)
- Convert **dynamic pressure** to:
  - Velocity (m/s)
  - Mach number
- Supports dynamic pressure input in **Pascal (Pa)** or **PSI**

---

## Assumptions and Constants

The calculations are based on the following fixed constants:

| Parameter      | Value          | Description           |
| -------------- | -------------- | --------------------- |
| Air density    | `1.2 kg/m³`    | Constant air density  |
| Speed of sound | `343 m/s`      | Speed of sound in air |
| PSI to Pascal  | `6894.7572932` | Conversion factor     |

These values are hard-coded and are not configurable via CLI arguments.

---

## Installation

Clone the repository and build using Cargo:

```bash
cargo build --release
```
