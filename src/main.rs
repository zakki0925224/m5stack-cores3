#![no_std]
#![no_main]

extern crate alloc;

use crate::board::camera::*;
use alloc::vec;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use esp_hal::clock::CpuClock;
use esp_hal::main;

mod board;
mod delay;
mod drivers;
mod imu;
mod panic;
mod print;
mod time;

esp_bootloader_esp_idf::esp_app_desc!();

fn yuv_to_rgb565(y: u8, cb: u8, cr: u8) -> Rgb565 {
    let y = y as i32;
    let cb = cb as i32 - 128;
    let cr = cr as i32 - 128;

    let r = (y + ((359 * cr) >> 8)).clamp(0, 255) as u8;
    let g = (y - ((88 * cb + 183 * cr) >> 8)).clamp(0, 255) as u8;
    let b = (y + ((454 * cb) >> 8)).clamp(0, 255) as u8;

    Rgb565::new(r >> 3, g >> 2, b >> 3)
}

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

    let mut frame_buffer = vec![0u8; FRAME_SIZE];

    loop {
        // if let Some(touch_point) = board.read_touch() {
        //     println!("[{}]: {:?}", board.read_time(), touch_point);
        // }

        let captured = board.camera.capture(&mut frame_buffer);
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
        board.display.fill_contiguous(&area, colors).unwrap();
    }
}
