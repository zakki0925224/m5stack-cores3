// LTR-553ALS-WA

use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x23;
const REG_ALS_CONTR: u8 = 0x80;
const REG_PS_CONTR: u8 = 0x81;
const REG_ALS_CH1_0: u8 = 0x88;
const REG_PS_DATA_0: u8 = 0x8d;

pub fn init(i2c: &mut impl I2c) {
    write(i2c, REG_ALS_CONTR, 0x00);
    write(i2c, REG_PS_CONTR, 0x00);

    write(i2c, 0x82, 0x7f); // 60kHz, 100%, 50mA
    write(i2c, 0x83, 0x01); // 1 pulse
    write(i2c, 0x84, 0x02); // 100ms measurement rate

    write(i2c, 0x85, 0x03); // 100ms integration, 500ms rate

    write(i2c, REG_ALS_CONTR, 0x01); // ALS active, gain x1
    write(i2c, REG_PS_CONTR, 0x02); // PS active (bit1のみ)

    crate::delay::delay_ms(600);
}

pub fn read_als(i2c: &mut impl I2c) -> u16 {
    let mut buf = [0u8; 4];
    i2c.write_read(ADDR_I2C, &[REG_ALS_CH1_0], &mut buf)
        .unwrap();
    let ch1 = u16::from_le_bytes([buf[0], buf[1]]);
    let ch0 = u16::from_le_bytes([buf[2], buf[3]]);
    (ch1 as u32 + ch0 as u32) as u16 / 2
}

pub fn read_proximity(i2c: &mut impl I2c) -> u16 {
    let lo = read(i2c, REG_PS_DATA_0);
    let hi = read(i2c, REG_PS_DATA_0 + 1);
    u16::from_le_bytes([lo, hi]) & 0x07ff
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
    i2c.write(ADDR_I2C, &[reg, val]).unwrap();
}

fn read(i2c: &mut impl I2c, reg: u8) -> u8 {
    let mut buf = [0u8; 1];
    i2c.write_read(ADDR_I2C, &[reg], &mut buf).unwrap();
    buf[0]
}
