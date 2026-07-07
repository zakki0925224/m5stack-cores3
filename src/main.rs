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
use esp_hal::dma::DmaRxBuf;
use esp_hal::dma_buffers;
use esp_hal::main;
use esp_println::println;

mod board;
mod delay;
mod drivers;
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

    let (rx_buf, rx_descs, _, _) = dma_buffers!(1024, 0);
    let dma_rx_buf = DmaRxBuf::new(rx_descs, rx_buf).unwrap();
    let transfer = board.camera.receive(dma_rx_buf).map_err(|e| e.0).unwrap();
    let (result, camera, dma_rx_buf) = transfer.wait();
    result.unwrap();
    board.camera = camera;
    println!("{:?}", &dma_rx_buf.as_slice()[..20]);

    loop {
        // if let Some(touch_point) = board.read_touch() {
        //     println!("[{}]: {:?}", board.read_time(), touch_point);
        // }
    }
}
