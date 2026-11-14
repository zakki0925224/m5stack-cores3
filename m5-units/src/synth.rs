// M5Stack Unit Synth (U178), MIDI over UART (31250 baud, 8N1, TX only)

use embedded_io::Write;

pub const BAUDRATE: u32 = 31_250;

pub const ACOUSTIC_GRAND_PIANO: u8 = 0; // GM program number, 0-based

const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const CONTROL_CHANGE: u8 = 0xb0;
const PROGRAM_CHANGE: u8 = 0xc0;
const SYSEX_START: u8 = 0xf0;
const SYSEX_END: u8 = 0xf7;
const SYSTEM_RESET: u8 = 0xff;

const CC_BANK_SELECT: u8 = 0x00;
const CC_VOLUME: u8 = 0x07;
const CC_ALL_NOTES_OFF: u8 = 0x7b;

pub fn set_instrument<W: Write>(w: &mut W, ch: u8, bank: u8, program: u8) -> Result<(), W::Error> {
    control_change(w, ch, CC_BANK_SELECT, bank)?;
    w.write_all(&[status(PROGRAM_CHANGE, ch), program & 0x7f])
}

pub fn note_on<W: Write>(w: &mut W, ch: u8, note: u8, velocity: u8) -> Result<(), W::Error> {
    w.write_all(&[status(NOTE_ON, ch), note & 0x7f, velocity & 0x7f])
}

pub fn note_off<W: Write>(w: &mut W, ch: u8, note: u8, velocity: u8) -> Result<(), W::Error> {
    w.write_all(&[status(NOTE_OFF, ch), note & 0x7f, velocity & 0x7f])
}

pub fn all_notes_off<W: Write>(w: &mut W, ch: u8) -> Result<(), W::Error> {
    control_change(w, ch, CC_ALL_NOTES_OFF, 0)
}

/// 0..=127
pub fn set_volume<W: Write>(w: &mut W, ch: u8, level: u8) -> Result<(), W::Error> {
    control_change(w, ch, CC_VOLUME, level)
}

/// 0..=127, universal SysEx master volume
pub fn set_master_volume<W: Write>(w: &mut W, level: u8) -> Result<(), W::Error> {
    w.write_all(&[SYSEX_START, 0x7f, 0x7f, 0x04, 0x01, 0x00, level & 0x7f, SYSEX_END])
}

pub fn reset<W: Write>(w: &mut W) -> Result<(), W::Error> {
    w.write_all(&[SYSTEM_RESET])
}

fn control_change<W: Write>(w: &mut W, ch: u8, cc: u8, val: u8) -> Result<(), W::Error> {
    w.write_all(&[status(CONTROL_CHANGE, ch), cc & 0x7f, val & 0x7f])
}

fn status(kind: u8, ch: u8) -> u8 {
    kind | (ch & 0x0f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::Infallible;
    use embedded_io::ErrorType;
    use std::vec::Vec;

    #[derive(Default)]
    struct Recorder(Vec<u8>);

    impl ErrorType for Recorder {
        type Error = Infallible;
    }

    impl Write for Recorder {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Infallible> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Infallible> {
            Ok(())
        }
    }

    #[test]
    fn note_on_masks_channel_and_data() {
        let mut w = Recorder::default();
        note_on(&mut w, 0x12, 0xbc, 0xff).unwrap();
        assert_eq!(w.0, [0x92, 0x3c, 0x7f]);
    }

    #[test]
    fn note_off_keeps_velocity() {
        let mut w = Recorder::default();
        note_off(&mut w, 1, 60, 64).unwrap();
        assert_eq!(w.0, [0x81, 60, 64]);
    }

    #[test]
    fn set_instrument_selects_bank_then_program() {
        let mut w = Recorder::default();
        set_instrument(&mut w, 0, 0, ACOUSTIC_GRAND_PIANO).unwrap();
        assert_eq!(w.0, [0xb0, 0x00, 0x00, 0xc0, 0x00]);
    }
}
