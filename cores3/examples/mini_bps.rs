#![no_std]
#![no_main]

use cores3::{
    board::CoreS3,
    delay::{BusyWait, delay_ms},
    error::Result,
    m5_units::{
        self,
        mini_bps::{self, MiniBps},
    },
    println,
};
use esp_hal::{clock::CpuClock, main};

esp_bootloader_esp_idf::esp_app_desc!();

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

    assert!(
        m5_units::scan(i2c)
            .find(|addr| *addr == mini_bps::DEFAULT_ADDR)
            .is_some()
    );
    assert_eq!(
        mini_bps::read_chip_id(i2c, mini_bps::DEFAULT_ADDR)?,
        mini_bps::CHIP_ID
    );

    let bps = MiniBps::new(i2c, &mut BusyWait, mini_bps::DEFAULT_ADDR)?;

    loop {
        let m = bps.read(i2c)?;
        println!("{:.2} C, {:.2} hPa", m.temperature, m.pressure / 100.0);
        delay_ms(1000);
    }
}
