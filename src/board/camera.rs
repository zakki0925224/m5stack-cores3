use esp_hal::{
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

pub type Cam = Camera<'static>;

#[allow(clippy::too_many_arguments)]
pub fn init(
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
) -> Cam {
    let config = Config::default().with_frequency(Rate::from_mhz(20));
    let lcd_cam = LcdCam::new(lcd_cam);

    Camera::new(lcd_cam.cam, dma_channel, config)
        .unwrap()
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
        .with_data7(d7)
}
