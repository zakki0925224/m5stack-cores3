# m5stack-cores3

Board support for the M5Stack CoreS3, built from scratch on `esp-hal`.

## Usage

```toml
[dependencies]
cores3 = { git = "https://github.com/zakki0925224/m5stack-cores3", features = ["global-alloc", "panic-handler"] }
```

The binary also needs the usual esp-hal setup (not inherited from dependencies): `rust-toolchain.toml` (`channel = "esp"`), `.cargo/config.toml` (Xtensa target, `build-std`), and `build.rs` (`-Tlinkall.x`).

## Development

- `cargo test` — host tests for `cores3-core`
- `cargo check-board` — check `cores3` for Xtensa
- `cargo run-board <example>` — flash an example from `cores3/examples/` and monitor

## Implementation Status

### Sensors & Peripherals

- [x] Touch IPS LCD (ILI9342C) - display output
- [x] RTC (BM8563) - time read/write
- [x] Proximity & ALS Sensor (LTR-553ALS-WA) - ambient light, proximity
- [x] Power Management (AXP2101) - battery voltage, VBUS, charging status, battery level
- [x] 6-axis IMU (BMI270) - accelerometer, gyroscope
- [x] 3-axis Magnetometer (BMM150) - via BMI270 AUX bus
- [x] Touch input (FT6336U) - capacitive touchscreen
- [x] Camera (GC0308 0.3MP)
- [ ] Speaker (AW88298) - 16-bit I2S amplifier
- [ ] Audio Codec (ES7210) - dual-microphone
- [ ] microSD slot

### Connectivity

- [ ] Wi-Fi (2.4GHz)
- [ ] USB OTG / CDC

## References

- [M5Stack CoreS3 Documentation](https://docs.m5stack.com/en/core/CoreS3)
- [M5Unified source code](https://github.com/m5stack/M5Unified)
