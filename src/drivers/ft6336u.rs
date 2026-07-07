// FT6336U capacitive touch controller

use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x38;
const REG_TD_STATUS: u8 = 0x02;

#[derive(Debug, Clone, Copy)]
pub struct TouchPoint {
    pub x: u16,
    pub y: u16,
}

pub fn read(i2c: &mut impl I2c) -> Option<TouchPoint> {
    let mut buf = [0u8; 5];
    i2c.write_read(ADDR_I2C, &[REG_TD_STATUS], &mut buf).ok()?;

    if buf[0] & 0x0f == 0 {
        return None;
    }

    let x = ((buf[1] & 0x0f) as u16) << 8 | buf[2] as u16;
    let y = ((buf[3] & 0x0f) as u16) << 8 | buf[4] as u16;

    Some(TouchPoint { x, y })
}
