// FT6336U capacitive touch controller

use crate::error::Result;
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x38;
const REG_TD_STATUS: u8 = 0x02;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchPoint {
    pub x: u16,
    pub y: u16,
}

pub fn read(i2c: &mut impl I2c) -> Result<Option<TouchPoint>> {
    let mut buf = [0u8; 5];
    super::read_regs(i2c, ADDR_I2C, REG_TD_STATUS, &mut buf)?;

    if buf[0] & 0x0f == 0 {
        return Ok(None);
    }

    let x = u16::from_be_bytes([buf[1] & 0x0f, buf[2]]);
    let y = u16::from_be_bytes([buf[3] & 0x0f, buf[4]]);

    Ok(Some(TouchPoint { x, y }))
}
