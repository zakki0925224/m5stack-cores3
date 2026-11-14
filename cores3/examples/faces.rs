#![no_std]
#![no_main]

use cores3::{
    board::CoreS3,
    delay::delay_ms,
    error::Result,
    m5_units::{
        self,
        faces::{self, Model, gamepad3, keyboard3},
    },
    print, println,
};
use esp_hal::{clock::CpuClock, main};

esp_bootloader_esp_idf::esp_app_desc!();

const ADDR: u8 = faces::DEFAULT_ADDR;

#[main]
fn main() -> ! {
    if let Err(e) = run() {
        panic!("{}", e);
    }
    unreachable!();
}

fn run() -> Result<()> {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut board = CoreS3::new(peripherals)?;
    let i2c = board.m_bus();

    for addr in m5_units::scan(i2c) {
        println!("found: {:#04x}", addr);
    }

    let model = faces::read_model(i2c, ADDR)?;
    println!("model: {:?}, firmware: {:#04x}", model, faces::read_firmware_version(i2c, ADDR)?);

    let mut prev = gamepad3::Buttons::default();
    loop {
        match model {
            Model::Gamepad3 => {
                if let Some(pressed) = gamepad3::read_buttons(i2c, ADDR)?
                    && pressed != prev
                {
                    println!("pressed: {:?}", pressed);
                    prev = pressed;
                }
            }
            Model::Keyboard3 => match keyboard3::read_key(i2c, ADDR)? {
                Some(keyboard3::Key::Char(c)) => print!("{}", c),
                Some(keyboard3::Key::Enter) => println!(),
                Some(key) => print!("[{:?}]", key),
                None => {}
            },
            _ => {}
        }
        delay_ms(5);
    }
}
