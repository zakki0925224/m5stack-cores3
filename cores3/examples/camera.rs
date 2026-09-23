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
        let full_source_rows = captured / (FRAME_WIDTH * 2);
        let out_rows = full_source_rows / 2;
        let buf = &frame_buffer[..captured];

        let colors = (0..out_rows).flat_map(|out_y| {
            let row_start = (out_y * 2) * FRAME_WIDTH * 2;
            (0..FRAME_WIDTH / 2).map(move |out_x| {
                let i = row_start + out_x * 4;
                yuv_to_rgb565(buf[i], buf[i + 1], buf[i + 3])
            })
        });
        let area = Rectangle::new(
            Point::zero(),
            Size::new((FRAME_WIDTH / 2) as u32, out_rows as u32),
        );
        board
            .display
            .fill_contiguous(&area, colors)
            .map_err(Error::hal)?;
    }
}
