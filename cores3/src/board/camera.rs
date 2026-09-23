use crate::error::{Error, ErrorKind, Result};
use esp_hal::{
    dma::{DmaDescriptor, DmaRxBuf},
    dma_buffers,
    lcd_cam::{
        LcdCam,
        cam::{Camera, Config},
    },
    peripherals::{
        DMA_CH0, GPIO2, GPIO15, GPIO16, GPIO38, GPIO39, GPIO40, GPIO41, GPIO42, GPIO45, GPIO46,
        GPIO47, GPIO48, LCD_CAM,
    },
    time::{Duration, Instant, Rate},
};

pub const FRAME_WIDTH: usize = 320;
pub const FRAME_HEIGHT: usize = 240;
pub const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * 2;

const DMA_BUF_SIZE: usize = FRAME_SIZE + 4092 * 2;

pub struct Cam {
    cam: Option<Camera<'static>>,
    buf: Option<DmaRxBuf>,
}

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

        let (rx_buffer, rx_descriptors, _, _) = dma_buffers!(DMA_BUF_SIZE, 0);
        let buf = DmaRxBuf::new(rx_descriptors, rx_buffer).map_err(Error::hal)?;

        Ok(Self {
            cam: Some(cam),
            buf: Some(buf),
        })
    }

    pub fn capture(&mut self, out: &mut [u8]) -> Result<usize> {
        const MAX_ATTEMPTS: u32 = 20;
        const FRAME_TIMEOUT: Duration = Duration::from_millis(1000);

        let mut cam = self
            .cam
            .take()
            .expect("camera driver lost by an earlier capture");
        let mut buf = self
            .buf
            .take()
            .expect("camera buffer lost by an earlier capture");

        for _ in 0..MAX_ATTEMPTS {
            let transfer = match cam.receive(buf) {
                Ok(transfer) => transfer,
                Err((e, c, b)) => {
                    self.cam = Some(c);
                    self.buf = Some(b);
                    return Err(Error::hal(e));
                }
            };

            let start = Instant::now();
            while !transfer.is_done() && start.elapsed() < FRAME_TIMEOUT {}
            let (c, b) = transfer.stop();
            cam = c;

            let (descriptors, buffer) = b.split();
            let frame_len = bytes_until_eof(descriptors);
            buf = DmaRxBuf::new(descriptors, buffer).map_err(Error::hal)?;

            if frame_len == Some(FRAME_SIZE) {
                let n = FRAME_SIZE.min(out.len());
                out[..n].copy_from_slice(&buf.as_slice()[..n]);
                self.cam = Some(cam);
                self.buf = Some(buf);
                return Ok(n);
            }
        }

        self.cam = Some(cam);
        self.buf = Some(buf);
        Err(Error::from(ErrorKind::Timeout)
            .with_context("camera could not deliver a complete frame"))
    }
}

fn bytes_until_eof(descriptors: &[DmaDescriptor]) -> Option<usize> {
    let mut total = 0;
    for desc in descriptors {
        total += desc.len();
        if desc.flags.suc_eof() {
            return Some(total);
        }
    }
    None
}
