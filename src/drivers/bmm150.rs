pub const ADDR_I2C: u8 = 0x10;
pub const REG_CHIP_ID: u8 = 0x40;
pub const REG_RESET: u8 = 0x4b;
pub const REG_CTRL: u8 = 0x4c;
pub const REG_DATA_X_LSB: u8 = 0x42;

pub const RESET_CMD: u8 = 0x83; // soft reset + power on
pub const NORMAL_MODE: u8 = 0x38; // normal mode / ODR 30Hz
pub const WHO_AM_I: u8 = 0x32;

pub fn decode(raw: &[u8; 6]) -> (i16, i16, i16) {
    let x = i16::from_le_bytes([raw[0], raw[1]]) >> 3;
    let y = i16::from_le_bytes([raw[2], raw[3]]) >> 3;
    let z = i16::from_le_bytes([raw[4], raw[5]]) >> 1;
    (x, y, z)
}
