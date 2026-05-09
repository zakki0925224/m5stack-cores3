use crate::board::display::CoreS3Display;
use esp_hal::{
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
};

pub mod aw9523;
pub mod axp2101;
pub mod display;

pub fn delay_ms(ms: u32) {
    for _ in 0..(ms * 50_000) {
        core::hint::black_box(());
    }
}

pub struct CoreS3 {
    pub display: CoreS3Display,
}

impl CoreS3 {
    pub fn new(peripherals: Peripherals) -> Self {
        let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
            .unwrap()
            .with_sda(peripherals.GPIO12)
            .with_scl(peripherals.GPIO11);

        aw9523::Aw9523::init(&mut i2c);
        aw9523::Aw9523::lcd_reset(&mut i2c);
        axp2101::Axp2101::enable_backlight(&mut i2c);

        let display = display::init(
            peripherals.SPI2,
            peripherals.GPIO37,
            peripherals.GPIO36,
            peripherals.GPIO3,
            peripherals.GPIO35,
        );

        Self { display }
    }
}
