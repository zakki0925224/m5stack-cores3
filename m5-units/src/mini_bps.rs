// M5Stack Unit Mini BPS v1.1 (U090-B), QMP6988 barometric pressure sensor

use crate::read_reg;
use embedded_hal::{delay::DelayNs, i2c::I2c};

pub const DEFAULT_ADDR: u8 = 0x70;
pub const CHIP_ID: u8 = 0x5c;

const REG_CALIB: u8 = 0xa0;
const REG_CHIP_ID: u8 = 0xd1;
const REG_RESET: u8 = 0xe0;
const REG_CONFIG: u8 = 0xf1;
const REG_CTRL_MEAS: u8 = 0xf4;
const REG_PRESS_MSB: u8 = 0xf7; // press[3] then temp[3]

const CALIB_LEN: usize = 25;

const MODE_NORMAL: u8 = 0b11;
const OSRS_1X: u8 = 1;
const OSRS_8X: u8 = 4;
const FILTER_4: u8 = 2;

pub fn read_chip_id<I: I2c>(i2c: &mut I, addr: u8) -> Result<u8, I::Error> {
    read_reg(i2c, addr, REG_CHIP_ID)
}

#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    pub temperature: f32, // degC
    pub pressure: f32,    // Pa
}

pub struct MiniBps {
    addr: u8,
    calib: Calib,
}

impl MiniBps {
    pub fn new<I: I2c, D: DelayNs>(i2c: &mut I, delay: &mut D, addr: u8) -> Result<Self, I::Error> {
        i2c.write(addr, &[REG_RESET, 0xe6])?;
        delay.delay_ms(20);
        i2c.write(addr, &[REG_RESET, 0x00])?;

        // read byte by byte like the vendor driver; burst read is untested
        let mut raw = [0u8; CALIB_LEN];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = read_reg(i2c, addr, REG_CALIB + i as u8)?;
        }

        i2c.write(addr, &[REG_CONFIG, FILTER_4])?;
        i2c.write(addr, &[REG_CTRL_MEAS, OSRS_1X << 5 | OSRS_8X << 2 | MODE_NORMAL])?;
        delay.delay_ms(20);

        Ok(Self {
            addr,
            calib: Calib::parse(&raw),
        })
    }

    pub fn read<I: I2c>(&self, i2c: &mut I) -> Result<Measurement, I::Error> {
        let mut buf = [0u8; 6];
        i2c.write_read(self.addr, &[REG_PRESS_MSB], &mut buf)?;

        let dp = raw24(&buf[0..3]);
        let dt = raw24(&buf[3..6]);
        let tx = self.calib.temperature(dt);
        let pr = self.calib.pressure(dp, tx);

        Ok(Measurement {
            temperature: tx as f32 / 256.0,
            pressure: pr as f32 / 16.0,
        })
    }
}

fn raw24(b: &[u8]) -> i64 {
    let v = (b[0] as i64) << 16 | (b[1] as i64) << 8 | b[2] as i64;
    v - (1 << 23)
}

// coefficients already converted to the vendor's fixed-point form
#[derive(Debug, PartialEq)]
struct Calib {
    a0: i64,
    a1: i64,
    a2: i64,
    b00: i64,
    bt1: i64,
    bt2: i64,
    bp1: i64,
    b11: i64,
    bp2: i64,
    b12: i64,
    b21: i64,
    bp3: i64,
}

impl Calib {
    fn parse(d: &[u8; CALIB_LEN]) -> Self {
        let s16 = |i: usize| i16::from_be_bytes([d[i], d[i + 1]]) as i64;
        let s20 = |hi: u8, mid: u8, lo4: u8| {
            let v = (hi as u32) << 12 | (mid as u32) << 4 | lo4 as u32;
            (((v << 12) as i32) >> 12) as i64 // sign-extend 20 bits
        };

        Self {
            a0: s20(d[18], d[19], d[24] & 0x0f),
            b00: s20(d[0], d[1], d[24] >> 4),
            a1: 3608 * s16(20) - 1731677965,
            a2: 16889 * s16(22) - 87619360,
            bt1: 2982 * s16(2) + 107370906,
            bt2: 329854 * s16(4) + 108083093,
            bp1: 19923 * s16(6) + 1133836764,
            b11: 2406 * s16(8) + 118215883,
            bp2: 3079 * s16(10) - 181579595,
            b12: 6846 * s16(12) + 85590281,
            b21: 13836 * s16(14) + 79333336,
            bp3: 2915 * s16(16) + 157155561,
        }
    }

    // 1/256 degC
    fn temperature(&self, dt: i64) -> i64 {
        let wk1 = self.a1 * dt;
        let wk2 = (self.a2 * dt) >> 14;
        let wk2 = (wk2 * dt) >> 10;
        let wk2 = ((wk1 + wk2) / 32767) >> 19;
        ((self.a0 + wk2) >> 4) as i16 as i64
    }

    // 1/16 Pa
    fn pressure(&self, dp: i64, tx: i64) -> i64 {
        let mut wk1 = self.bt1 * tx;
        wk1 += (self.bp1 * dp) >> 5;

        let mut wk3 = (((self.bt2 * tx) >> 1) * tx) >> 8;
        wk3 += (((self.b11 * tx) >> 4) * dp) >> 1;
        wk3 += (((self.bp2 * dp) >> 13) * dp) >> 1;
        wk1 += wk3 >> 14;

        let mut wk3 = ((((self.b12 * tx) * tx) >> 22) * dp) >> 1;
        wk3 += (((((self.b21 * tx) >> 6) * dp) >> 23) * dp) >> 1;
        wk3 += ((((self.bp3 * dp) >> 12) * dp) >> 23) * dp;
        wk1 += wk3 >> 15;

        ((wk1 / 32767) >> 11) + self.b00
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw24_is_offset_by_2_pow_23() {
        assert_eq!(raw24(&[0x80, 0x00, 0x00]), 0);
        assert_eq!(raw24(&[0x00, 0x00, 0x00]), -(1 << 23));
        assert_eq!(raw24(&[0xff, 0xff, 0xff]), (1 << 23) - 1);
    }

    #[test]
    fn calib_sign_extends_20bit_coefficients() {
        let mut d = [0u8; CALIB_LEN];
        d[18] = 0xff; // a0 = 0xffff_f -> -1
        d[19] = 0xff;
        d[24] = 0x0f;
        d[0] = 0x7f; // b00 = 0x7fff_0 -> max positive with low nibble 0
        d[1] = 0xff;

        let c = Calib::parse(&d);
        assert_eq!(c.a0, -1);
        assert_eq!(c.b00, 0x7fff0);
    }

    #[test]
    fn calib_reads_16bit_coefficients_big_endian() {
        let mut d = [0u8; CALIB_LEN];
        d[20] = 0xff; // a1 raw = -1
        d[21] = 0xff;
        d[2] = 0x00; // bt1 raw = 1
        d[3] = 0x01;

        let c = Calib::parse(&d);
        assert_eq!(c.a1, -3608 - 1731677965);
        assert_eq!(c.bt1, 2982 + 107370906);
    }
}
