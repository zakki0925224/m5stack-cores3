// LTR-553ALS-WA ambient light / proximity sensor

use crate::{delay::delay_ms, error::Result};
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x23;
const REG_ALS_CONTR: u8 = 0x80;
const REG_PS_CONTR: u8 = 0x81;
const REG_PS_LED: u8 = 0x82;
const REG_PS_N_PULSES: u8 = 0x83;
const REG_PS_MEAS_RATE: u8 = 0x84;
const REG_ALS_MEAS_RATE: u8 = 0x85;
const REG_ALS_DATA_CH1_0: u8 = 0x88;
const REG_PS_DATA_0: u8 = 0x8d;

pub fn init(i2c: &mut impl I2c) -> Result<()> {
    write(i2c, REG_ALS_CONTR, 0x00)?;
    write(i2c, REG_PS_CONTR, 0x00)?;

    write(i2c, REG_PS_LED, 0x7f)?; // 60kHz, 100%, 50mA
    write(i2c, REG_PS_N_PULSES, 0x01)?; // 1 pulse
    write(i2c, REG_PS_MEAS_RATE, 0x02)?; // 100ms measurement rate

    write(i2c, REG_ALS_MEAS_RATE, 0x03)?; // 100ms integration, 500ms rate

    write(i2c, REG_ALS_CONTR, 0x01)?; // ALS active, gain x1
    write(i2c, REG_PS_CONTR, 0x02)?; // PS active (bit 1 only)

    delay_ms(600);

    Ok(())
}

pub fn read_als(i2c: &mut impl I2c) -> Result<u16> {
    let mut buf = [0u8; 4];
    super::read_regs(i2c, ADDR_I2C, REG_ALS_DATA_CH1_0, &mut buf)?;
    let ch1 = u16::from_le_bytes([buf[0], buf[1]]) as u32;
    let ch0 = u16::from_le_bytes([buf[2], buf[3]]) as u32;
    Ok(((ch0 + ch1) / 2) as u16)
}

pub fn read_proximity(i2c: &mut impl I2c) -> Result<u16> {
    let lo = read(i2c, REG_PS_DATA_0)?;
    let hi = read(i2c, REG_PS_DATA_0 + 1)?;
    Ok(u16::from_le_bytes([lo, hi]) & 0x07ff)
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) -> Result<()> {
    super::write_reg(i2c, ADDR_I2C, reg, val)
}

fn read(i2c: &mut impl I2c, reg: u8) -> Result<u8> {
    super::read_reg(i2c, ADDR_I2C, reg)
}
