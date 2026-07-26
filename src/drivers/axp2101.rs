use crate::error::{Error, Result};
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x34;

pub fn init(i2c: &mut impl I2c) -> Result<()> {
    write(i2c, 0x92, 13)?; // ALDO1 = 1.8V
    write(i2c, 0x93, 28)?; // ALDO2 = 3.3V
    write(i2c, 0x94, 28)?; // ALDO3 = 3.3V
    write(i2c, 0x95, 28)?; // ALDO4 = 3.3V
    write(i2c, 0x99, 28)?; // DLDO1 = 3.3V (LCD BL)
    write(i2c, 0x90, 0xbf)?; // enable all LDOs
    write(i2c, 0x30, 0x0f)?; // enable ADC channels

    // wait for LDOs stabilize
    crate::delay::delay_ms(10);

    Ok(())
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) -> Result<()> {
    i2c.write(ADDR_I2C, &[reg, val]).map_err(Error::hal)?;
    Ok(())
}

pub fn read_battery_mv(i2c: &mut impl I2c) -> Result<u16> {
    let mut buf = [0u8; 2];
    i2c.write_read(ADDR_I2C, &[0x34], &mut buf)
        .map_err(Error::hal)?;
    Ok(((buf[0] as u16 & 0x3f) << 8) | buf[1] as u16) // mV
}

pub fn read_battery_level(i2c: &mut impl I2c) -> Result<i8> {
    Ok(read(i2c, 0xa4)? as i8) // 0-100%
}

pub fn is_charging(i2c: &mut impl I2c) -> Result<bool> {
    Ok(read(i2c, 0x01)? & 0x04 != 0)
}

pub fn read_vbus_mv(i2c: &mut impl I2c) -> Result<u16> {
    let mut buf = [0u8; 2];
    i2c.write_read(ADDR_I2C, &[0x38], &mut buf)
        .map_err(Error::hal)?;
    Ok(((buf[0] as u16 & 0x3f) << 8) | buf[1] as u16) // mV
}

fn read(i2c: &mut impl I2c, reg: u8) -> Result<u8> {
    let mut buf = [0u8; 1];
    i2c.write_read(ADDR_I2C, &[reg], &mut buf)
        .map_err(Error::hal)?;
    Ok(buf[0])
}
