// AW9523B I/O expander

use crate::{
    delay::{BusyWait, delay_ms},
    error::{Error, Result},
};
use embedded_hal::{delay::DelayNs, i2c::I2c};

const ADDR_I2C: u8 = 0x58;
const REG_OUTPUT_P0: u8 = 0x02;
const REG_OUTPUT_P1: u8 = 0x03;
const REG_CONFIG_P0: u8 = 0x04;
const REG_CONFIG_P1: u8 = 0x05;
const REG_GCR: u8 = 0x11;
const REG_LEDMODE_P0: u8 = 0x12;
const REG_LEDMODE_P1: u8 = 0x13;

const P0_BUS_OUT_EN: u8 = 1 << 1;
const P1_BOOST_EN: u8 = 1 << 7;
const P1_RST: u8 = 0b11; // TP_RST | LCD_RST

pub fn init(i2c: &mut impl I2c) -> Result<()> {
    // set output levels first so TP_RST/LCD_RST don't glitch low once enabled below
    write(i2c, REG_OUTPUT_P0, 0b00000101)?; // P0.2 = SPK_EN
    write(i2c, REG_OUTPUT_P1, 0b00000011)?; // P1.0 = TP_RST, P1.1 = LCD_RST (both released)

    write(i2c, REG_CONFIG_P0, 0b00011000)?; // 1 = input (camera/other), 0 = output
    write(i2c, REG_CONFIG_P1, 0b00001100)?;

    write(i2c, REG_GCR, 0b00010000)?; // port 0 push-pull mode

    write(i2c, REG_LEDMODE_P0, 0xff)?; // GPIO mode, not LED constant-current mode
    write(i2c, REG_LEDMODE_P1, 0xff)?;

    Ok(())
}

pub fn reset_lcd(i2c: &mut impl I2c) -> Result<()> {
    // read-modify-write: keep other P1 pins (e.g. BOOST_EN)
    let p1 = read(i2c, REG_OUTPUT_P1)?;
    write(i2c, REG_OUTPUT_P1, p1 & !P1_RST)?;
    delay_ms(50);
    write(i2c, REG_OUTPUT_P1, p1 | P1_RST)?;
    delay_ms(200);
    Ok(())
}

pub fn enable_port_a_power(i2c: &mut impl I2c) -> Result<()> {
    let p0 = read(i2c, REG_OUTPUT_P0)?;
    if p0 & P0_BUS_OUT_EN != 0 {
        return Ok(());
    }

    let p1 = read(i2c, REG_OUTPUT_P1)?;
    write(i2c, REG_OUTPUT_P1, p1 | P1_BOOST_EN)?;

    for i in 0..8 {
        write(i2c, REG_OUTPUT_P0, p0 | P0_BUS_OUT_EN)?;
        BusyWait.delay_us(i * 16);
        write(i2c, REG_OUTPUT_P0, p0)?;
        BusyWait.delay_us(1000);
    }

    write(i2c, REG_OUTPUT_P0, p0 | P0_BUS_OUT_EN)?;
    if read(i2c, REG_OUTPUT_P0)? & P0_BUS_OUT_EN == 0 {
        return Err(Error::hal("BUS_OUT_EN not set"));
    }
    Ok(())
}

fn read(i2c: &mut impl I2c, reg: u8) -> Result<u8> {
    super::read_reg(i2c, ADDR_I2C, reg)
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) -> Result<()> {
    super::write_reg(i2c, ADDR_I2C, reg, val)
}
