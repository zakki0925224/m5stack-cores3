//! Drivers for M5Stack units -- board-independent, builds and unit-tests on the host.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod bytebutton;
pub mod mini_bps;
pub mod synth;

use embedded_hal::i2c::I2c;

pub fn scan<I: I2c>(i2c: &mut I) -> impl Iterator<Item = u8> + '_ {
    (0x08..=0x77).filter(move |&addr| i2c.write(addr, &[]).is_ok())
}
