use crate::{imu::Imu, time::Time};
use esp_hal::{
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
};

pub mod aw9523;
pub mod axp2101;
pub mod bm8563;
pub mod bmi270;
pub mod display;
pub mod ltr553;

pub fn delay_ms(ms: u32) {
    for _ in 0..(ms * 50_000) {
        core::hint::black_box(());
    }
}

pub struct CoreS3 {
    pub display: display::CoreS3Display,
    aw9523: aw9523::Aw9523,
    ltr553: ltr553::Ltr553AlsWa,
    i2c: I2c<'static, esp_hal::Blocking>,
}

impl CoreS3 {
    pub fn new(peripherals: Peripherals) -> Self {
        let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
            .unwrap()
            .with_sda(peripherals.GPIO12)
            .with_scl(peripherals.GPIO11);

        // power on all sensors
        axp2101::init(&mut i2c);

        bmi270::init(&mut i2c);

        let aw9523 = aw9523::Aw9523::init(&mut i2c);
        aw9523.lcd_reset(&mut i2c);

        let display = display::init(
            peripherals.SPI2,
            peripherals.GPIO37,
            peripherals.GPIO36,
            peripherals.GPIO3,
            peripherals.GPIO35,
        );

        let ltr553 = ltr553::Ltr553AlsWa::init(&mut i2c);

        Self {
            display,
            aw9523,
            ltr553,
            i2c,
        }
    }

    pub fn read_time(&mut self) -> Time {
        bm8563::read_time(&mut self.i2c)
    }

    pub fn set_time(&mut self, time: Time) {
        bm8563::set_time(&mut self.i2c, time);
    }

    pub fn wait_seconds(&mut self, seconds: u32) {
        let start = self.read_time().to_total_seconds();

        loop {
            let now = self.read_time().to_total_seconds();
            let elapsed = if now >= start {
                now - start
            } else {
                now + 86400 - start // midnight rollover
            };

            if elapsed >= seconds {
                break;
            }
        }
    }

    pub fn read_als(&mut self) -> u16 {
        self.ltr553.read_als(&mut self.i2c)
    }

    pub fn read_proximity(&mut self) -> u16 {
        self.ltr553.read_proximity(&mut self.i2c)
    }

    pub fn read_battery_mv(&mut self) -> u16 {
        axp2101::read_battery_mv(&mut self.i2c)
    }

    pub fn read_battery_level(&mut self) -> i8 {
        axp2101::read_battery_level(&mut self.i2c)
    }

    pub fn is_charging(&mut self) -> bool {
        axp2101::is_charging(&mut self.i2c)
    }

    pub fn read_vbus_mv(&mut self) -> u16 {
        axp2101::read_vbus_mv(&mut self.i2c)
    }

    pub fn read_imu(&mut self) -> Imu {
        bmi270::read_imu(&mut self.i2c)
    }
}
