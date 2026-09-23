use crate::{
    board::{camera::Cam, speaker::Speaker},
    delay::delay_ms,
    drivers::{aw9523, aw88298, axp2101, bm8563, bmi270, ft6336u, gc0308, ltr553},
    error::Result,
    imu::Imu,
    time::Time,
};
use esp_hal::{
    gpio::{DriveMode, Flex, InputConfig, OutputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::{GPIO11, GPIO12, Peripherals},
};

pub mod camera;
pub mod display;
pub mod speaker;
pub mod uart;

pub struct CoreS3 {
    pub display: display::Display,
    pub camera: camera::Cam,
    speaker: speaker::Speaker,
    i2c: I2c<'static, esp_hal::Blocking>,
}

impl CoreS3 {
    pub fn new(peripherals: Peripherals) -> Result<Self> {
        #[cfg(feature = "global-alloc")]
        crate::heap::init_psram(peripherals.PSRAM);

        let (scl, sda) = recover_i2c_bus(peripherals.GPIO11, peripherals.GPIO12);
        let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())?
            .with_sda(sda)
            .with_scl(scl);

        // power on all sensors
        axp2101::init(&mut i2c)?;

        bmi270::init(&mut i2c)?;

        aw9523::init(&mut i2c)?;
        aw9523::reset_lcd(&mut i2c)?;

        ltr553::init(&mut i2c)?;

        let display = display::init(
            peripherals.SPI2,
            peripherals.GPIO37,
            peripherals.GPIO36,
            peripherals.GPIO3,
            peripherals.GPIO35,
        )?;

        let camera = Cam::new(
            peripherals.LCD_CAM,
            peripherals.DMA_CH0,
            peripherals.GPIO2,
            peripherals.GPIO45,
            peripherals.GPIO46,
            peripherals.GPIO38,
            peripherals.GPIO39,
            peripherals.GPIO40,
            peripherals.GPIO41,
            peripherals.GPIO42,
            peripherals.GPIO15,
            peripherals.GPIO16,
            peripherals.GPIO48,
            peripherals.GPIO47,
        )?;

        gc0308::init(&mut i2c)?;
        gc0308::set_frame_size(
            &mut i2c,
            camera::FRAME_WIDTH as u16,
            camera::FRAME_HEIGHT as u16,
        )?;

        let speaker = Speaker::new(
            peripherals.I2S0,
            peripherals.DMA_CH1,
            peripherals.GPIO34,
            peripherals.GPIO33,
            peripherals.GPIO13,
        )?;
        aw88298::init(&mut i2c, speaker::SAMPLE_RATE)?;

        Ok(Self {
            display,
            camera,
            speaker,
            i2c,
        })
    }

    pub fn read_time(&mut self) -> Result<Time> {
        bm8563::read_time(&mut self.i2c)
    }

    pub fn set_time(&mut self, time: Time) -> Result<()> {
        bm8563::set_time(&mut self.i2c, time)
    }

    pub fn wait_seconds(&mut self, seconds: u32) -> Result<()> {
        let start = self.read_time()?.to_total_seconds();

        loop {
            let now = self.read_time()?.to_total_seconds();
            let elapsed = if now >= start {
                now - start
            } else {
                now + 86400 - start // midnight rollover
            };

            if elapsed >= seconds {
                break;
            }
        }

        Ok(())
    }

    pub fn read_als(&mut self) -> Result<u16> {
        ltr553::read_als(&mut self.i2c)
    }

    pub fn read_proximity(&mut self) -> Result<u16> {
        ltr553::read_proximity(&mut self.i2c)
    }

    pub fn read_battery_mv(&mut self) -> Result<u16> {
        axp2101::read_battery_mv(&mut self.i2c)
    }

    pub fn read_battery_level(&mut self) -> Result<i8> {
        axp2101::read_battery_level(&mut self.i2c)
    }

    pub fn is_charging(&mut self) -> Result<bool> {
        axp2101::is_charging(&mut self.i2c)
    }

    pub fn read_vbus_mv(&mut self) -> Result<u16> {
        axp2101::read_vbus_mv(&mut self.i2c)
    }

    pub fn read_imu(&mut self) -> Result<Imu> {
        bmi270::read_imu(&mut self.i2c)
    }

    pub fn read_touch(&mut self) -> Result<Option<ft6336u::TouchPoint>> {
        ft6336u::read(&mut self.i2c)
    }

    pub fn play(&mut self, samples: &[i16]) -> Result<()> {
        self.speaker.play(&mut self.i2c, samples)
    }

    pub fn play_iter(&mut self, samples: impl Iterator<Item = i16>) -> Result<()> {
        self.speaker.play_iter(&mut self.i2c, samples)
    }

    pub fn tone(&mut self, freq_hz: u32, duration_ms: u32) -> Result<()> {
        self.speaker.tone(&mut self.i2c, freq_hz, duration_ms)
    }

    /// 0 = -96dB, 255 = 0dB
    pub fn set_speaker_volume(&mut self, volume: u8) -> Result<()> {
        aw88298::set_volume(&mut self.i2c, volume)
    }

    pub fn set_speaker_mute(&mut self, mute: bool) -> Result<()> {
        aw88298::set_mute(&mut self.i2c, mute)
    }
}

fn recover_i2c_bus(scl: GPIO11<'static>, sda: GPIO12<'static>) -> (Flex<'static>, Flex<'static>) {
    let mut scl = Flex::new(scl);
    let mut sda = Flex::new(sda);

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

    let od = OutputConfig::default().with_drive_mode(DriveMode::OpenDrain);
    let pu = InputConfig::default().with_pull(Pull::Up);
    for pin in [&mut scl, &mut sda] {
        pin.apply_output_config(&od);
        pin.apply_input_config(&pu);
        pin.set_input_enable(true);
        pin.set_output_enable(true);
        pin.set_high();
    }

    (scl, sda)
}
