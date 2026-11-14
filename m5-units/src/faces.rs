// M5Stack Faces3 panels on Faces Bottom3 (A168), I2C via M-Bus

use crate::read_reg;
use embedded_hal::i2c::I2c;

pub const DEFAULT_ADDR: u8 = 0x08; // 0x54 is the IAP bootloader, leave it alone

const REG_KEY: u8 = 0x00;
const REG_MODEL_ID: u8 = 0xd0;
const REG_FIRMWARE_VERSION: u8 = 0xfe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Calculator3,
    Keyboard3,
    Gamepad3,
    Unknown(u8),
}

pub fn read_model<I: I2c>(i2c: &mut I, addr: u8) -> Result<Model, I::Error> {
    Ok(match read_reg(i2c, addr, REG_MODEL_ID)? {
        0x01 => Model::Calculator3,
        0x02 => Model::Keyboard3,
        0x03 => Model::Gamepad3,
        id => Model::Unknown(id),
    })
}

pub fn read_firmware_version<I: I2c>(i2c: &mut I, addr: u8) -> Result<u8, I::Error> {
    read_reg(i2c, addr, REG_FIRMWARE_VERSION)
}

pub mod gamepad3 {
    use super::*;

    /// discriminant = bit position (checked on hardware; vendor header marks it unverified)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Button {
        Up,
        Down,
        Left,
        Right,
        A,
        B,
        Select,
        Start,
    }

    impl Button {
        pub const ALL: [Self; 8] = [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::A,
            Self::B,
            Self::Select,
            Self::Start,
        ];

        fn mask(self) -> u8 {
            1 << self as u8
        }
    }

    /// set of pressed buttons
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct Buttons(u8);

    impl Buttons {
        pub fn is_pressed(self, button: Button) -> bool {
            self.0 & button.mask() != 0
        }

        pub fn is_empty(self) -> bool {
            self.0 == 0
        }

        pub fn iter(self) -> impl Iterator<Item = Button> {
            Button::ALL.into_iter().filter(move |&b| self.is_pressed(b))
        }
    }

    impl FromIterator<Button> for Buttons {
        fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {
            Self(iter.into_iter().fold(0, |acc, b| acc | b.mask()))
        }
    }

    impl core::fmt::Debug for Buttons {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    }

    /// `None` = no new sample (stale 0x00 when polled without INT)
    pub fn read_buttons<I: I2c>(i2c: &mut I, addr: u8) -> Result<Option<Buttons>, I::Error> {
        let raw = read_reg(i2c, addr, REG_KEY)?;
        Ok((raw != 0x00).then_some(Buttons(!raw))) // active low
    }
}

pub mod keyboard3 {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Key {
        Char(char),
        Backspace,
        Tab,
        Enter,
        Esc,
        Delete,
        /// modifiers (0x80..), ALT/FN combos (144..=193)
        Other(u8),
    }

    /// Normal mode. `None` = no key
    pub fn read_key<I: I2c>(i2c: &mut I, addr: u8) -> Result<Option<Key>, I::Error> {
        Ok(match read_reg(i2c, addr, REG_KEY)? {
            0x00 | 0xff => None, // 0xff = no mapping
            0x08 => Some(Key::Backspace),
            0x09 => Some(Key::Tab),
            0x0d => {
                read_reg(i2c, addr, REG_KEY)?; // Enter is CR LF; consume LF to release IRQ
                Some(Key::Enter)
            }
            0x1b => Some(Key::Esc),
            0x7f => Some(Key::Delete),
            c @ 0x20..=0x7e => Some(Key::Char(c as char)),
            code => Some(Key::Other(code)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        gamepad3::{Button, Buttons},
        keyboard3::Key,
        *,
    };
    use embedded_hal::i2c::{ErrorKind, ErrorType, Operation};
    use std::{collections::VecDeque, vec::Vec};

    #[derive(Default)]
    struct Replay {
        reads: VecDeque<u8>,
        regs: Vec<u8>,
    }

    impl Replay {
        fn new(reads: &[u8]) -> Self {
            Self {
                reads: reads.iter().copied().collect(),
                ..Default::default()
            }
        }
    }

    impl ErrorType for Replay {
        type Error = ErrorKind;
    }

    impl I2c for Replay {
        fn transaction(&mut self, _addr: u8, ops: &mut [Operation<'_>]) -> Result<(), ErrorKind> {
            for op in ops {
                match op {
                    Operation::Write(bytes) => self.regs.extend_from_slice(bytes),
                    Operation::Read(buf) => buf.fill(self.reads.pop_front().ok_or(ErrorKind::Other)?),
                }
            }
            Ok(())
        }
    }

    #[test]
    fn gamepad_is_active_low_and_ignores_stale_zero() {
        let mut i2c = Replay::new(&[0xff, !0b0001_0001, 0x00]); // A + Up pressed
        assert_eq!(gamepad3::read_buttons(&mut i2c, DEFAULT_ADDR), Ok(Some(Buttons::default())));

        let pressed = gamepad3::read_buttons(&mut i2c, DEFAULT_ADDR).unwrap().unwrap();
        assert_eq!(pressed, [Button::Up, Button::A].into_iter().collect());
        assert!(pressed.is_pressed(Button::A));
        assert!(!pressed.is_pressed(Button::B));
        assert!(pressed.iter().eq([Button::Up, Button::A]));

        assert_eq!(gamepad3::read_buttons(&mut i2c, DEFAULT_ADDR), Ok(None));
    }

    #[test]
    fn button_bit_order_matches_hardware() {
        for (bit, button) in Button::ALL.into_iter().enumerate() {
            let mut i2c = Replay::new(&[!(1u8 << bit)]);
            let pressed = gamepad3::read_buttons(&mut i2c, DEFAULT_ADDR).unwrap().unwrap();
            assert!(pressed.iter().eq([button]));
        }
    }

    #[test]
    fn keyboard_enter_consumes_lf() {
        let mut i2c = Replay::new(&[0x0d, 0x0a, b'a']);
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(Some(Key::Enter)));
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(Some(Key::Char('a'))));
        assert!(i2c.reads.is_empty());
    }

    #[test]
    fn keyboard_maps_codes() {
        let mut i2c = Replay::new(&[0x00, 0xff, 0x08, 0x80]);
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(None));
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(None));
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(Some(Key::Backspace)));
        assert_eq!(keyboard3::read_key(&mut i2c, DEFAULT_ADDR), Ok(Some(Key::Other(0x80))));
    }

    #[test]
    fn model_id_is_read_from_0xd0() {
        let mut i2c = Replay::new(&[0x03]);
        assert_eq!(read_model(&mut i2c, DEFAULT_ADDR), Ok(Model::Gamepad3));
        assert_eq!(i2c.regs, [REG_MODEL_ID]);
    }
}
