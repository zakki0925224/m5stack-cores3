#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use cores3::{
    board::{CoreS3, camera::*},
    color::yuv_to_rgb565,
    error::{Error, Result},
    println,
};
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
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

    Rectangle::new(Point::zero(), Size::new(320, 240))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut board.display)
        .map_err(Error::hal)?;

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Hello, CoreS3!", Point::new(10, 30), style)
        .draw(&mut board.display)
        .map_err(Error::hal)?;

    println!("Done!");

    let mut frame_buffer = vec![0u8; FRAME_SIZE];

    loop {
        let captured = board.camera.capture(&mut frame_buffer)?;
        let rows = (captured / (FRAME_WIDTH * 2)).min(FRAME_HEIGHT);
        let buf = &frame_buffer[..rows * FRAME_WIDTH * 2];

        let colors = buf.chunks_exact(4).flat_map(|p| {
            [
                yuv_to_rgb565(p[0], p[1], p[3]),
                yuv_to_rgb565(p[2], p[1], p[3]),
            ]
        });
        let area = Rectangle::new(Point::zero(), Size::new(FRAME_WIDTH as u32, rows as u32));
        board
            .display
            .fill_contiguous(&area, colors)
            .map_err(Error::hal)?;
    }
}
