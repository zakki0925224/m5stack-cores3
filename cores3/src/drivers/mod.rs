use crate::error::{Error, Result};
use embedded_hal::i2c::I2c;

pub mod aw88298;
pub mod aw9523;
pub mod axp2101;
pub mod bm8563;
pub mod bmi270;
pub mod bmm150;
pub mod ft6336u;
pub mod gc0308;
pub mod ltr553;

fn write_reg(i2c: &mut impl I2c, addr: u8, reg: u8, val: u8) -> Result<()> {
    i2c.write(addr, &[reg, val]).map_err(Error::hal)
}

fn read_regs(i2c: &mut impl I2c, addr: u8, reg: u8, buf: &mut [u8]) -> Result<()> {
    i2c.write_read(addr, &[reg], buf).map_err(Error::hal)
}

fn read_reg(i2c: &mut impl I2c, addr: u8, reg: u8) -> Result<u8> {
    let mut buf = [0u8; 1];
    read_regs(i2c, addr, reg, &mut buf)?;
    Ok(buf[0])
}
