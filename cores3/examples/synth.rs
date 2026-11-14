#![no_std]
#![no_main]

use cores3::{board::CoreS3, delay::delay_ms, error::Result, m5_units::synth, println};
use esp_hal::{clock::CpuClock, main, uart::Config as UartConfig};

esp_bootloader_esp_idf::esp_app_desc!();

const CH: u8 = 0;
// C4 .. C5
const SCALE: [u8; 8] = [60, 62, 64, 65, 67, 69, 71, 72];

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
    let uart = board.port_c();
    uart.apply_config(&UartConfig::default().with_baudrate(synth::BAUDRATE))?;

    synth::set_instrument(uart, CH, 0, synth::ACOUSTIC_GRAND_PIANO)?;

    loop {
        for note in SCALE {
            println!("note: {}", note);
            synth::note_on(uart, CH, note, 100)?;
            delay_ms(400);
            synth::note_off(uart, CH, note, 0)?;
            delay_ms(100);
        }
        delay_ms(1000);
    }
}
