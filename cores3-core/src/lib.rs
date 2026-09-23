//! Hardware-independent parts of `cores3` -- builds and unit-tests on the host.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod color;
pub mod heap;
pub mod time;
