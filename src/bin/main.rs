#![no_std]
#![no_main]

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_println::println;
use m5stack_cores3::board::CoreS3;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut board = CoreS3::new(peripherals);

    Rectangle::new(Point::zero(), Size::new(320, 240))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut board.display)
        .unwrap();

    println!("Done!");

    loop {}
}
