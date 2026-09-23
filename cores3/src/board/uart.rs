// reference: https://github.com/esp-rs/esp-println

pub struct Uart;

impl Uart {
    const ADDR_TX_ONE_CHAR: usize = 0x4000_0648;
    const ADDR_TX_FLUSH: usize = 0x4000_0690;
    const ADDR_GET_CHANNEL: usize = 0x4000_1a58;
    const ADDR_G_USB_PRINT: usize = 0x3fce_ffb8;

    pub const CHUNK_SIZE: usize = 32;

    pub fn tx_byte(b: u8) {
        unsafe {
            let tx_one_char: unsafe extern "C" fn(u8) -> i32 =
                core::mem::transmute(Self::ADDR_TX_ONE_CHAR);
            tx_one_char(b);
        }
    }

    pub fn flush() {
        unsafe {
            let tx_flush: unsafe extern "C" fn(u8) = core::mem::transmute(Self::ADDR_TX_FLUSH);
            let get_channel: unsafe extern "C" fn() -> u8 =
                core::mem::transmute(Self::ADDR_GET_CHANNEL);
            let g_usb_print = Self::ADDR_G_USB_PRINT as *mut bool;

            let channel = if *g_usb_print { 4 } else { get_channel() };
            tx_flush(channel)
        }
    }
}
