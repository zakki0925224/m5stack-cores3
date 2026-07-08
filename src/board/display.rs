use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    spi::Mode as SpiMode,
    spi::master::{Config as SpiConfig, Spi},
    time::Rate,
};
use mipidsi::{
    Builder, NoResetPin,
    interface::SpiInterface,
    models::ILI9342CRgb565,
    options::{ColorInversion, ColorOrder},
};

pub struct BusyWaitDelay;

impl DelayNs for BusyWaitDelay {
    fn delay_ns(&mut self, ns: u32) {
        for _ in 0..(ns / 20) {
            core::hint::black_box(());
        }
    }
}

static mut SPI_BUF: [u8; 512] = [0u8; 512];

pub type Display = mipidsi::Display<
    SpiInterface<
        'static,
        ExclusiveDevice<Spi<'static, esp_hal::Blocking>, Output<'static>, BusyWaitDelay>,
        Output<'static>,
    >,
    ILI9342CRgb565,
    NoResetPin,
>;

pub fn init(
    spi_peripheral: esp_hal::peripherals::SPI2<'static>,
    mosi: esp_hal::peripherals::GPIO37<'static>,
    sck: esp_hal::peripherals::GPIO36<'static>,
    cs_pin: esp_hal::peripherals::GPIO3<'static>,
    dc_pin: esp_hal::peripherals::GPIO35<'static>,
) -> Display {
    let dc = Output::new(dc_pin, Level::Low, OutputConfig::default());

    let spi = Spi::new(
        spi_peripheral,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(27))
            .with_mode(SpiMode::_0),
    )
    .unwrap()
    .with_mosi(mosi)
    .with_sck(sck);

    let cs = Output::new(cs_pin, Level::High, OutputConfig::default());
    let spi_device = ExclusiveDevice::new(spi, cs, BusyWaitDelay).unwrap();

    let buf = unsafe { (&raw mut SPI_BUF).as_mut().unwrap() };
    let spi_interface = SpiInterface::new(spi_device, dc, buf);

    Builder::new(ILI9342CRgb565, spi_interface)
        .color_order(ColorOrder::Bgr)
        .invert_colors(ColorInversion::Inverted)
        .init(&mut BusyWaitDelay)
        .unwrap()
}
