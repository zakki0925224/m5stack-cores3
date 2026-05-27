use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x58;
const REG_OUTPUT_P0: u8 = 0x02;
const REG_OUTPUT_P1: u8 = 0x03;
const REG_CONFIG_P0: u8 = 0x04;
const REG_CONFIG_P1: u8 = 0x05;
const REG_GCR: u8 = 0x11;

const REG_LEDMODE_P0: u8 = 0x12;
const REG_LEDMODE_P1: u8 = 0x13;

pub struct Aw9523;

impl Aw9523 {
    pub fn init(i2c: &mut impl I2c) -> Self {
        Self::write(i2c, REG_OUTPUT_P0, 0b00000101); // P0[0]=1, P0[2]=SPK_EN=1
        Self::write(i2c, REG_OUTPUT_P1, 0b00000011); // P1[0]=TP_RST=1, P1[1]=LCD_RST=1
        // P0[3,4] and P1[2,3] are inputs (camera/other); rest are outputs
        Self::write(i2c, REG_CONFIG_P0, 0b00011000);
        Self::write(i2c, REG_CONFIG_P1, 0b00001100);
        // P0 push-pull mode
        Self::write(i2c, REG_GCR, 0b00010000);
        // switch all pins to GPIO mode; from this point output=1 means HIGH
        Self::write(i2c, REG_LEDMODE_P0, 0xFF);
        Self::write(i2c, REG_LEDMODE_P1, 0xFF);
        // P1[0]=HIGH (TP_RST released), P1[1]=HIGH (LCD_RST released)
        Self
    }

    pub fn lcd_reset(&self, i2c: &mut impl I2c) {
        // GPIO mode: 0=LOW (assert active-LOW reset), 1=HIGH (release)
        Self::write(i2c, REG_OUTPUT_P1, 0x00); // TP_RST=LOW, LCD_RST=LOW
        crate::board::delay_ms(50);
        Self::write(i2c, REG_OUTPUT_P1, 0x03); // TP_RST=HIGH, LCD_RST=HIGH
        crate::board::delay_ms(200);
    }

    fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
        i2c.write(ADDR, &[reg, val]).unwrap();
    }
}
