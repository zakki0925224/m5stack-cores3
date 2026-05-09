#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_println::println;

mod board;
mod imu;
mod panic;
mod time;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut board = board::CoreS3::new(peripherals);

    Rectangle::new(Point::zero(), Size::new(320, 240))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut board.display)
        .unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Hello, CoreS3!", Point::new(10, 30), style)
        .draw(&mut board.display)
        .unwrap();

    println!("Done!");

    loop {
        let imu = board.read_imu();

        println!(
            "[{}]: ALS=0x{:x}, PS=0x{:x}, BMV={}mV, BLV={}%, CHG={}, VBUSMV={}mV, ACL: x={}, y={}, z={}, GYR: x={}, y={}, z={}, MAG: x={}, y={}, z={}",
            board.read_time(),
            board.read_als(),
            board.read_proximity(),
            board.read_battery_mv(),
            board.read_battery_level(),
            board.is_charging(),
            board.read_vbus_mv(),
            imu.accel.x,
            imu.accel.y,
            imu.accel.z,
            imu.gyro.x,
            imu.gyro.y,
            imu.gyro.z,
            imu.mag.x,
            imu.mag.y,
            imu.mag.z
        );
    }
}
