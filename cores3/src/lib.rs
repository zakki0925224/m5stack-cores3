//! Board support for the M5Stack CoreS3. See the README for the binary-side setup: build config is not inherited.

#![no_std]

extern crate alloc;

pub mod board;
pub mod delay;
pub mod drivers;
pub mod error;
pub mod heap;
pub mod imu;
#[cfg(feature = "panic-handler")]
mod panic;
pub mod print;

pub use cores3_core::{color, time};
pub use embedded_graphics;
pub use embedded_hal;
pub use esp_hal;
pub use mipidsi;
