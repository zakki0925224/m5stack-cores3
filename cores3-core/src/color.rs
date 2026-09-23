use embedded_graphics::pixelcolor::Rgb565;

pub fn yuv_to_rgb565(y: u8, cb: u8, cr: u8) -> Rgb565 {
    let y = y as i32;
    let cb = cb as i32 - 128;
    let cr = cr as i32 - 128;

    let r = (y + ((359 * cr) >> 8)).clamp(0, 255) as u8;
    let g = (y - ((88 * cb + 183 * cr) >> 8)).clamp(0, 255) as u8;
    let b = (y + ((454 * cb) >> 8)).clamp(0, 255) as u8;

    Rgb565::new(r >> 3, g >> 2, b >> 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::prelude::*;

    #[test]
    fn neutral_chroma_is_gray() {
        let c = yuv_to_rgb565(0, 128, 128);
        assert_eq!((c.r(), c.g(), c.b()), (0, 0, 0));

        let c = yuv_to_rgb565(255, 128, 128);
        assert_eq!((c.r(), c.g(), c.b()), (31, 63, 31));
    }

    #[test]
    fn chroma_pushes_the_expected_channel() {
        let red = yuv_to_rgb565(128, 128, 255);
        assert_eq!((red.r(), red.g(), red.b()), (31, 9, 16));

        let blue = yuv_to_rgb565(128, 255, 128);
        assert_eq!((blue.r(), blue.g(), blue.b()), (16, 21, 31));
    }

    #[test]
    fn out_of_range_results_are_clamped_not_wrapped() {
        // red would be 433 here, blue -227 in the next one
        assert_eq!(yuv_to_rgb565(255, 0, 255).r(), 31);
        assert_eq!(yuv_to_rgb565(0, 0, 128).b(), 0);
    }
}
