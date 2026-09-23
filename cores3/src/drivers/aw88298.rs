// AW88298 audio amplifier

use crate::error::{Error, ErrorKind, Result};
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x36;
const CHIP_ID: u16 = 0x1852;
const REG_ID: u8 = 0x00;
const REG_SYSCTRL: u8 = 0x04;
const REG_SYSCTRL2: u8 = 0x05;
const REG_I2SCTRL: u8 = 0x06;
const REG_HAGCCFG4: u8 = 0x0c;
const REG_BSTCTRL2: u8 = 0x61;

const SYSCTRL2_HMUTE: u16 = 1 << 4;
const VOL_MIN_ATTENUATION: u16 = 0xc0; // -96dB, 0x00 = 0dB

// I2SCTRL.I2SSR index = position in this table
const SAMPLE_RATES: [u32; 10] = [
    8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 96000,
];

pub fn init(i2c: &mut impl I2c, sample_rate: u32) -> Result<()> {
    let rate_index = SAMPLE_RATES
        .iter()
        .position(|&r| r == sample_rate)
        .ok_or_else(|| {
            Error::from(ErrorKind::InvalidArgument).with_context("unsupported sample rate")
        })?;

    if read(i2c, REG_ID)? != CHIP_ID {
        return Err(Error::from(ErrorKind::NotInitialized).with_context("AW88298 not found"));
    }

    write(i2c, REG_BSTCTRL2, 0x0673)?; // boost mode disabled
    set_enabled(i2c, false)?;
    write(i2c, REG_SYSCTRL2, 0x0008)?; // RMSE=0 HAGCE=0 HDCCE=0 HMUTE=0
    write(i2c, REG_I2SCTRL, 0x14c0 | rate_index as u16)?; // Philips, 16bit x 2ch
    set_volume(i2c, u8::MAX)?;

    Ok(())
}

pub fn set_enabled(i2c: &mut impl I2c, enabled: bool) -> Result<()> {
    if enabled {
        write(i2c, REG_SYSCTRL, 0x4040) // I2SEN=1 AMPPD=0 PWDN=0
    } else {
        write(i2c, REG_SYSCTRL, 0x4003) // I2SEN=0 AMPPD=1 PWDN=1
    }
}

/// 0 = -96dB, 255 = 0dB
pub fn set_volume(i2c: &mut impl I2c, volume: u8) -> Result<()> {
    let attenuation = (u8::MAX - volume) as u16 * VOL_MIN_ATTENUATION / u8::MAX as u16;
    write(i2c, REG_HAGCCFG4, (attenuation << 8) | 0x64)
}

pub fn set_mute(i2c: &mut impl I2c, mute: bool) -> Result<()> {
    let val = read(i2c, REG_SYSCTRL2)?;
    let val = if mute {
        val | SYSCTRL2_HMUTE
    } else {
        val & !SYSCTRL2_HMUTE
    };
    write(i2c, REG_SYSCTRL2, val)
}

// registers are 16-bit big-endian
fn write(i2c: &mut impl I2c, reg: u8, val: u16) -> Result<()> {
    let [hi, lo] = val.to_be_bytes();
    i2c.write(ADDR_I2C, &[reg, hi, lo]).map_err(Error::hal)
}

fn read(i2c: &mut impl I2c, reg: u8) -> Result<u16> {
    let mut buf = [0u8; 2];
    super::read_regs(i2c, ADDR_I2C, reg, &mut buf)?;
    Ok(u16::from_be_bytes(buf))
}
