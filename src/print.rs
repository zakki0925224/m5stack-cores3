// reference: https://github.com/esp-rs/esp-println

use crate::board::uart::Uart;

pub struct Printer;

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Printer {
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(Uart::CHUNK_SIZE) {
            for &b in chunk {
                Uart::tx_byte(b);
            }

            Uart::flush();
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            writeln!($crate::print::Printer, $($arg)*).ok();
        }
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            write!($crate::print::Printer, $($arg)*).ok();
        }
    }};
}
