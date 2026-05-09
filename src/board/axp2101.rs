use embedded_hal::i2c::I2c;

const AXP2101_ADDR: u8 = 0x34;

pub struct Axp2101;

impl Axp2101 {
    pub fn enable_backlight(i2c: &mut impl I2c) {
        Self::write(i2c, 0x99, 0x1c);
        let val = Self::read(i2c, 0x90);
        Self::write(i2c, 0x90, val | 0x80);
    }

    fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
        i2c.write(AXP2101_ADDR, &[reg, val]).ok();
    }

    fn read(i2c: &mut impl I2c, reg: u8) -> u8 {
        let mut buf = [0u8; 1];
        i2c.write_read(AXP2101_ADDR, &[reg], &mut buf).ok();
        buf[0]
    }
}
