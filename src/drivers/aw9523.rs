use embedded_hal::i2c::I2c;

const ADDR_I2C: u8 = 0x58;
pub const REG_OUTPUT_P0: u8 = 0x02;
pub const REG_OUTPUT_P1: u8 = 0x03;
pub const REG_CONFIG_P0: u8 = 0x04;
pub const REG_CONFIG_P1: u8 = 0x05;
pub const REG_GCR: u8 = 0x11;

pub const REG_LEDMODE_P0: u8 = 0x12;
pub const REG_LEDMODE_P1: u8 = 0x13;

pub fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
    i2c.write(ADDR_I2C, &[reg, val]).unwrap()
}

pub fn init(i2c: &mut impl I2c) {
    // set output levels first so TP_RST/LCD_RST don't glitch low once enabled below
    write(i2c, REG_OUTPUT_P0, 0b00000101); // P0.2 = SPK_EN
    write(i2c, REG_OUTPUT_P1, 0b00000011); // P1.0 = TP_RST, P1.1 = LCD_RST (both released)

    write(i2c, REG_CONFIG_P0, 0b00011000); // 1 = input (camera/other), 0 = output
    write(i2c, REG_CONFIG_P1, 0b00001100);

    write(i2c, REG_GCR, 0b00010000); // port 0 push-pull mode

    write(i2c, REG_LEDMODE_P0, 0xff); // GPIO mode, not LED constant-current mode
    write(i2c, REG_LEDMODE_P1, 0xff);
}

pub fn reset_lcd(i2c: &mut impl I2c) {
    write(i2c, REG_OUTPUT_P1, 0x00);
    crate::delay::delay_ms(50);
    write(i2c, REG_OUTPUT_P1, 0x03);
    crate::delay::delay_ms(200);
}
