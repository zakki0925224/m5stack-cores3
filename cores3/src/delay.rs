use embedded_hal::delay::DelayNs;
use esp_hal::delay::Delay;

/// timer-based, independent of CPU clock
pub struct BusyWait;

impl DelayNs for BusyWait {
    fn delay_ns(&mut self, ns: u32) {
        Delay::new().delay_nanos(ns);
    }
}

pub fn delay_ms(ms: u32) {
    BusyWait.delay_ms(ms);
}
