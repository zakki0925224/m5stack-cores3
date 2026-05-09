use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x34;

pub fn init(i2c: &mut impl I2c) {
    write(i2c, 0x92, 13); // ALDO1 = 1.8V
    write(i2c, 0x93, 28); // ALDO2 = 3.3V
    write(i2c, 0x94, 28); // ALDO3 = 3.3V
    write(i2c, 0x95, 28); // ALDO4 = 3.3V
    write(i2c, 0x99, 28); // DLDO1 = 3.3V (LCD BL)
    write(i2c, 0x90, 0xbf); // enable all LDOs

    // wait for LDOs stabilize
    crate::board::delay_ms(10);
}

fn write(i2c: &mut impl I2c, reg: u8, val: u8) {
    i2c.write(ADDR, &[reg, val]).unwrap();
}
