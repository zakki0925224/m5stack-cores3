#![no_std]
#![no_main]

use cores3::{
    board::CoreS3,
    error::Result,
    m5_units::{self, bytebutton},
    println,
};
use esp_hal::{clock::CpuClock, main};

esp_bootloader_esp_idf::esp_app_desc!();

const ADDR: u8 = bytebutton::DEFAULT_ADDR;

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
    let i2c = board.port_a();

    for addr in m5_units::scan(i2c) {
        println!("found: {:#04x}", addr);
    }

    println!(
        "firmware: {:#04x}",
        bytebutton::read_firmware_version(i2c, ADDR)?
    );

    bytebutton::set_user_led_mode(i2c, ADDR)?;
    for led in 0..9 {
        bytebutton::set_brightness(i2c, ADDR, led, 255)?;
        bytebutton::set_rgb888(i2c, ADDR, led, 0x00ff0000)?; // red
    }

    let mut prev = None;
    loop {
        let buttons = bytebutton::read_buttons(i2c, ADDR)?;
        if prev != Some(buttons) {
            println!("pressed: {:08b}", buttons);
            prev = Some(buttons);
        }
    }
}
