#![no_std]
#![no_main]

use cores3::{board::CoreS3, error::Result, println};
use esp_hal::{clock::CpuClock, main};

esp_bootloader_esp_idf::esp_app_desc!();

// C4 .. C5
const SCALE_HZ: [u32; 8] = [262, 294, 330, 349, 392, 440, 494, 523];

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
    board.set_speaker_volume(64)?;

    for freq in SCALE_HZ {
        board.tone(freq, 200)?;
    }

    println!("Touch the screen to play a note");

    loop {
        if let Some(point) = board.read_touch()? {
            // left edge = C4, right edge = C5
            let index = (point.x as usize * SCALE_HZ.len() / 320).min(SCALE_HZ.len() - 1);
            board.tone(SCALE_HZ[index], 100)?;
        }
    }
}
