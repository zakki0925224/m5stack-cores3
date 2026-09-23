// BM8563 RTC

use crate::{
    error::{Error, Result},
    time::{Time, bcd_to_dec, dec_to_bcd},
};
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x51;
const REG_SECONDS: u8 = 0x02;

pub fn read_time(i2c: &mut impl I2c) -> Result<Time> {
    let mut buf = [0u8; 3];
    super::read_regs(i2c, ADDR_I2C, REG_SECONDS, &mut buf)?;

    Ok(Time {
        hours: bcd_to_dec(buf[2] & 0x3f),
        minutes: bcd_to_dec(buf[1] & 0x7f),
        seconds: bcd_to_dec(buf[0] & 0x7f),
    })
}

pub fn set_time(i2c: &mut impl I2c, time: Time) -> Result<()> {
    // burst write seconds/minutes/hours from REG_SECONDS
    i2c.write(
        ADDR_I2C,
        &[
            REG_SECONDS,
            dec_to_bcd(time.seconds),
            dec_to_bcd(time.minutes),
            dec_to_bcd(time.hours),
        ],
    )
    .map_err(Error::hal)
}
