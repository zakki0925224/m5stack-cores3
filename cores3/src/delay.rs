use embedded_hal::delay::DelayNs;

const NS_PER_ITER: u32 = 20;

pub struct BusyWait;

impl DelayNs for BusyWait {
    fn delay_ns(&mut self, ns: u32) {
        for _ in 0..(ns / NS_PER_ITER) {
            core::hint::black_box(());
        }
    }
}

pub fn delay_ms(ms: u32) {
    BusyWait.delay_ms(ms);
}
