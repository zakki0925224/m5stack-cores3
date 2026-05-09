use embedded_hal::i2c::I2c;

const AW9523_ADDR: u8 = 0x58;
#[allow(dead_code)]
const REG_OUTPUT_P0: u8 = 0x02;
const REG_OUTPUT_P1: u8 = 0x03;
const REG_CONFIG_P0: u8 = 0x04;
const REG_CONFIG_P1: u8 = 0x05;
const REG_GCR: u8 = 0x11;

pub struct Aw9523;

impl Aw9523 {
    pub fn init(i2c: &mut impl I2c) {
        Self::write(i2c, REG_GCR, 0x10);
        Self::write(i2c, REG_CONFIG_P0, 0x00);
        Self::write(i2c, REG_CONFIG_P1, 0x00);
    }

    pub fn lcd_reset(i2c: &mut impl I2c) {
        Self::write(i2c, REG_OUTPUT_P1, 0x00); // RST LOW
        crate::board::delay_ms(50);
        Self::write(i2c, REG_OUTPUT_P1, 0x02); // RST HIGH
        crate::board::delay_ms(200);
    }

    fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
        i2c.write(AW9523_ADDR, &[reg, val]).ok();
    }
}
