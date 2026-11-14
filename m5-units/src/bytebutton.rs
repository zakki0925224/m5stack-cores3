// M5Stack Unit ByteButton (U192)

use crate::read_reg;
use embedded_hal::i2c::I2c;

pub const DEFAULT_ADDR: u8 = 0x47;
pub const STATUS_LED: u8 = 8; // near the connector

const REG_STATUS: u8 = 0x00;
const REG_LED_BRIGHTNESS: u8 = 0x10; // + n
const REG_LED_SHOW_MODE: u8 = 0x19;
const REG_RGB888: u8 = 0x20; // + 4n
const REG_FIRMWARE_VERSION: u8 = 0xfe;

pub fn read_firmware_version<I: I2c>(i2c: &mut I, addr: u8) -> Result<u8, I::Error> {
    read_reg(i2c, addr, REG_FIRMWARE_VERSION)
}

/// bit n = button n (7 = leftmost), 1 = pressed
pub fn read_buttons<I: I2c>(i2c: &mut I, addr: u8) -> Result<u8, I::Error> {
    read_reg(i2c, addr, REG_STATUS).map(|raw| !raw) // active low
}

pub fn set_user_led_mode<I: I2c>(i2c: &mut I, addr: u8) -> Result<(), I::Error> {
    i2c.write(addr, &[REG_LED_SHOW_MODE, 0]) // 0 = user defined, 1 = default
}

pub fn set_brightness<I: I2c>(i2c: &mut I, addr: u8, led: u8, val: u8) -> Result<(), I::Error> {
    i2c.write(addr, &[REG_LED_BRIGHTNESS + led, val])
}

pub fn set_rgb888<I: I2c>(i2c: &mut I, addr: u8, led: u8, rgb: u32) -> Result<(), I::Error> {
    let [b0, b1, b2, b3] = rgb.to_le_bytes();
    i2c.write(addr, &[REG_RGB888 + led * 4, b0, b1, b2, b3])
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::i2c::{ErrorKind, ErrorType, Operation};
    use std::vec::Vec;

    #[derive(Default)]
    struct Recorder {
        writes: Vec<(u8, Vec<u8>)>,
        read_value: u8,
    }

    impl ErrorType for Recorder {
        type Error = ErrorKind;
    }

    impl I2c for Recorder {
        fn transaction(&mut self, addr: u8, ops: &mut [Operation<'_>]) -> Result<(), ErrorKind> {
            for op in ops {
                match op {
                    Operation::Write(bytes) => self.writes.push((addr, bytes.to_vec())),
                    Operation::Read(buf) => buf.fill(self.read_value),
                }
            }
            Ok(())
        }
    }

    #[test]
    fn rgb888_is_sent_little_endian() {
        let mut i2c = Recorder::default();
        set_rgb888(&mut i2c, DEFAULT_ADDR, 2, 0x00_11_22_33).unwrap();
        assert_eq!(
            i2c.writes,
            [(DEFAULT_ADDR, std::vec![0x28, 0x33, 0x22, 0x11, 0x00])]
        );
    }

    #[test]
    fn buttons_are_active_low() {
        let mut i2c = Recorder {
            read_value: 0b0111_1110, // leftmost and rightmost pressed
            ..Default::default()
        };
        assert_eq!(read_buttons(&mut i2c, DEFAULT_ADDR).unwrap(), 0b1000_0001);
    }
}
