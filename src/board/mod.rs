use crate::{imu::Imu, time::Time};
use esp_hal::{
    gpio::{DriveMode, Flex, InputConfig, OutputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
};

pub mod aw9523;
pub mod axp2101;
pub mod bm8563;
pub mod bmi270;
pub mod bmm150;
pub mod display;
pub mod ltr553;
pub mod touch;

pub fn delay_ms(ms: u32) {
    for _ in 0..(ms * 50_000) {
        core::hint::black_box(());
    }
}

pub struct CoreS3 {
    pub display: display::CoreS3Display,
    aw9523: aw9523::Aw9523,
    ltr553: ltr553::Ltr553AlsWa,
    touch: touch::Touch,
    i2c: I2c<'static, esp_hal::Blocking>,
}

impl CoreS3 {
    pub fn new(peripherals: Peripherals) -> Self {
        let (i2c_scl, i2c_sda) = {
            let mut scl = Flex::new(peripherals.GPIO11);
            let mut sda = Flex::new(peripherals.GPIO12);

            // push-pull output for recovery clocking
            scl.set_output_enable(true);
            scl.set_high();
            sda.set_input_enable(true);

            if sda.is_low() {
                for _ in 0..9 {
                    delay_ms(1);
                    scl.set_low();
                    delay_ms(1);
                    scl.set_high();
                }
                delay_ms(5);
            }

            // reconfigure both pins for I2C: open-drain + input + pull-up
            let od = OutputConfig::default().with_drive_mode(DriveMode::OpenDrain);
            let pu = InputConfig::default().with_pull(Pull::Up);
            scl.apply_output_config(&od);
            scl.apply_input_config(&pu);
            scl.set_input_enable(true);
            scl.set_output_enable(true);
            scl.set_high();
            sda.apply_output_config(&od);
            sda.apply_input_config(&pu);
            sda.set_input_enable(true);
            sda.set_output_enable(true);
            sda.set_high();

            (scl, sda)
        };

        let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
            .unwrap()
            .with_sda(i2c_sda)
            .with_scl(i2c_scl);

        // power on all sensors
        axp2101::init(&mut i2c);

        let result = bmi270::init(&mut i2c);
        assert!(result);

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
        let touch = touch::Touch::init(&mut i2c);

        Self {
            display,
            aw9523,
            ltr553,
            touch,
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

    pub fn read_touch(&mut self) -> Option<touch::TouchPoint> {
        self.touch.read(&mut self.i2c)
    }
}
