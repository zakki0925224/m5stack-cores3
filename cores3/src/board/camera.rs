use crate::error::{Error, ErrorKind, Result};
use esp_hal::{
    dma_rx_stream_buffer,
    lcd_cam::{
        LcdCam,
        cam::{Camera, Config},
    },
    peripherals::{
        DMA_CH0, GPIO2, GPIO15, GPIO16, GPIO38, GPIO39, GPIO40, GPIO41, GPIO42, GPIO45, GPIO46,
        GPIO47, GPIO48, LCD_CAM,
    },
    time::Rate,
};

// GC0308 default output: full VGA, YUV422 (2 bytes/pixel).
pub const FRAME_WIDTH: usize = 640;
pub const FRAME_HEIGHT: usize = 480;
pub const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * 2;

pub struct Cam(Option<Camera<'static>>);

impl Cam {
    pub fn new(
        lcd_cam: LCD_CAM<'static>,
        dma_channel: DMA_CH0<'static>,
        mclk: GPIO2<'static>,
        pclk: GPIO45<'static>,
        vsync: GPIO46<'static>,
        href: GPIO38<'static>,
        d0: GPIO39<'static>,
        d1: GPIO40<'static>,
        d2: GPIO41<'static>,
        d3: GPIO42<'static>,
        d4: GPIO15<'static>,
        d5: GPIO16<'static>,
        d6: GPIO48<'static>,
        d7: GPIO47<'static>,
    ) -> Result<Self> {
        let config = Config::default().with_frequency(Rate::from_mhz(20));
        let lcd_cam = LcdCam::new(lcd_cam);
        let cam = Camera::new(lcd_cam.cam, dma_channel, config)
            .map_err(Error::hal)?
            .with_master_clock(mclk)
            .with_pixel_clock(pclk)
            .with_vsync(vsync)
            .with_h_enable(href)
            .with_data0(d0)
            .with_data1(d1)
            .with_data2(d2)
            .with_data3(d3)
            .with_data4(d4)
            .with_data5(d5)
            .with_data6(d6)
            .with_data7(d7);
        Ok(Self(Some(cam)))
    }

    pub fn capture(&mut self, out: &mut [u8]) -> Result<usize> {
        const MAX_RESTARTS: u32 = 200;

        let mut cam = self
            .0
            .take()
            .expect("camera driver lost by an earlier capture");
        let mut total = 0;
        let mut restarts = 0u32;

        'restart: loop {
            let stream_buf = dma_rx_stream_buffer!(4092 * 64, 4092);
            let mut transfer = match cam.receive(stream_buf) {
                Ok(transfer) => transfer,
                Err((e, recovered, _buf)) => {
                    self.0 = Some(recovered);
                    return Err(Error::hal(e));
                }
            };

            loop {
                let (data, ends_with_eof) = transfer.peek_until_eof();
                if data.is_empty() {
                    if transfer.is_done() {
                        let (c, _) = transfer.stop();
                        cam = c;
                        restarts += 1;
                        if restarts > MAX_RESTARTS {
                            self.0 = Some(cam);
                            return Err(Error::from(ErrorKind::Timeout)
                                .with_context("camera produced no data"));
                        }
                        continue 'restart;
                    }
                    continue;
                }

                restarts = 0;

                let n = data.len().min(out.len() - total);
                out[total..total + n].copy_from_slice(&data[..n]);
                total += n;

                let peeked = data.len();
                transfer.consume(peeked);

                if ends_with_eof || total >= out.len() {
                    let (c, _) = transfer.stop();
                    self.0 = Some(c);
                    return Ok(total);
                }
            }
        }
    }
}
