// AXP2101 power management

use crate::{delay::delay_ms, error::Result};
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x34;
const REG_STATUS_2: u8 = 0x01;
const REG_ADC_ENABLE: u8 = 0x30;
const REG_VBAT_H: u8 = 0x34;
const REG_VBUS_H: u8 = 0x38;
const REG_LDO_ENABLE: u8 = 0x90;
const REG_BATTERY_PERCENT: u8 = 0xa4;

pub fn init(i2c: &mut impl I2c) -> Result<()> {
    write(i2c, 0x92, 13)?; // ALDO1 = 1.8V
    write(i2c, 0x93, 28)?; // ALDO2 = 3.3V
    write(i2c, 0x94, 28)?; // ALDO3 = 3.3V
    write(i2c, 0x95, 28)?; // ALDO4 = 3.3V
    write(i2c, 0x99, 28)?; // DLDO1 = 3.3V (LCD BL)
    write(i2c, REG_LDO_ENABLE, 0xbf)?; // enable all LDOs
    write(i2c, REG_ADC_ENABLE, 0x0f)?; // enable ADC channels

    // wait for LDOs stabilize
    delay_ms(10);

    Ok(())
}

pub fn read_battery_mv(i2c: &mut impl I2c) -> Result<u16> {
    read_adc_mv(i2c, REG_VBAT_H)
}

pub fn read_battery_level(i2c: &mut impl I2c) -> Result<i8> {
    Ok(read(i2c, REG_BATTERY_PERCENT)? as i8) // 0-100%
}

pub fn is_charging(i2c: &mut impl I2c) -> Result<bool> {
    Ok(read(i2c, REG_STATUS_2)? & 0x04 != 0)
}

pub fn read_vbus_mv(i2c: &mut impl I2c) -> Result<u16> {
    read_adc_mv(i2c, REG_VBUS_H)
}

// 14-bit big-endian ADC result, 1mV/LSB
fn read_adc_mv(i2c: &mut impl I2c, reg_h: u8) -> Result<u16> {
    let mut buf = [0u8; 2];
    super::read_regs(i2c, ADDR_I2C, reg_h, &mut buf)?;
    Ok(u16::from_be_bytes([buf[0] & 0x3f, buf[1]]))
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) -> Result<()> {
    super::write_reg(i2c, ADDR_I2C, reg, val)
}

fn read(i2c: &mut impl I2c, reg: u8) -> Result<u8> {
    super::read_reg(i2c, ADDR_I2C, reg)
}
