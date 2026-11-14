# m5stack-cores3

Board support for the M5Stack CoreS3, built from scratch on `esp-hal`.

## Usage

Build config is not inherited from dependencies, so the binary needs all of these. Use `esp_hal` and `m5_units` through the `cores3` re-exports to avoid duplicate versions.

`Cargo.toml`

```toml
[dependencies]
cores3                 = { git = "https://github.com/zakki0925224/m5stack-cores3", rev = "<commit>", features = ["global-alloc", "panic-handler"] }
esp-bootloader-esp-idf = { version = "0.5.0", features = ["esp32s3"] }

[profile.release]
codegen-units = 1
debug         = 2
lto           = "fat"
opt-level     = "s"
```

`rust-toolchain.toml` (install with `espup install -v 1.98.1.0 -a esp-1.98.1.0`; newer toolchains fail to link)

```toml
[toolchain]
channel = "esp-1.98.1.0"
```

`.cargo/config.toml`

```toml
[build]
target = "xtensa-esp32s3-none-elf"

[target.xtensa-esp32s3-none-elf]
runner    = "espflash flash --monitor --chip esp32s3"
rustflags = ["-C", "link-arg=-nostartfiles"]

[unstable]
build-std = ["core", "alloc"]
```

`build.rs`

```rust
fn main() {
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
```

`src/main.rs`

```rust
#![no_std]
#![no_main]

use cores3::{board::CoreS3, error::Result, esp_hal::{self, main}};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    if let Err(e) = run() {
        panic!("{}", e);
    }
    unreachable!();
}

fn run() -> Result<()> {
    let _board = CoreS3::new(esp_hal::init(esp_hal::Config::default()))?;
    loop {}
}
```

## Development

- `cargo test` — host tests for `cores3-core` and `m5-units`
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
- [x] Speaker (AW88298) - 16-bit I2S amplifier
- [ ] Audio Codec (ES7210) - dual-microphone
- [ ] microSD slot

### M5Stack Units (`m5-units`)

Board-independent drivers on `embedded-hal` / `embedded-io`. The bus is borrowed per call: `board.port_a()` (I2C), `board.port_c()` (UART), `board.m_bus()` (internal I2C).

- [x] ByteButton (U192) - Port A
- [x] Mini BPS v1.1 (U090-B, QMP6988) - Port A
- [x] Synth (U178) - Port C, MIDI
- [x] Faces3 Keyboard3 / Gamepad3 on Faces Bottom3 - M-Bus

### Connectivity

- [ ] Wi-Fi (2.4GHz)
- [ ] USB OTG / CDC

## References

- [M5Stack CoreS3 Documentation](https://docs.m5stack.com/en/core/CoreS3)
- [M5Unified source code](https://github.com/m5stack/M5Unified)
