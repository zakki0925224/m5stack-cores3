//! PSRAM-backed heap. The allocator itself lives in [`cores3_core::heap`].

pub use cores3_core::heap::{AllocationError, LinkedListAllocator};

pub fn psram_raw_parts(psram: esp_hal::peripherals::PSRAM<'static>) -> (*mut u8, usize) {
    let psram = esp_hal::psram::Psram::new(psram, esp_hal::psram::PsramConfig::default());
    psram.raw_parts()
}

#[cfg(feature = "global-alloc")]
mod global {
    use super::{LinkedListAllocator, psram_raw_parts};

    #[global_allocator]
    static ALLOCATOR: LinkedListAllocator = LinkedListAllocator::empty();

    pub fn init_psram(psram: esp_hal::peripherals::PSRAM<'static>) {
        let (start, size) = psram_raw_parts(psram);
        unsafe { ALLOCATOR.init(start, size) };
    }

    pub fn used() -> usize {
        ALLOCATOR.used()
    }
}

#[cfg(feature = "global-alloc")]
pub use global::{init_psram, used};
