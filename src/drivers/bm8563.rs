use crate::time::Time;
use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x51;
const REG_SECONDS: u8 = 0x02;

pub fn read_time(i2c: &mut impl I2c) -> Time {
    let mut buf = [0u8; 3];
    i2c.write_read(ADDR_I2C, &[REG_SECONDS], &mut buf).unwrap();

    Time {
        hours: bcd_to_dec(buf[2] & 0x3f),
        minutes: bcd_to_dec(buf[1] & 0x7f),
        seconds: bcd_to_dec(buf[0] & 0x7f),
    }
}

pub fn set_time(i2c: &mut impl I2c, time: Time) {
    i2c.write(
        ADDR_I2C,
        &[
            REG_SECONDS,
            dec_to_bcd(time.seconds),
            dec_to_bcd(time.minutes),
            dec_to_bcd(time.hours),
        ],
    )
    .unwrap();
}

fn bcd_to_dec(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0x0f)
}
fn dec_to_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}
