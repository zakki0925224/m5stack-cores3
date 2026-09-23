#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl core::fmt::Display for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}

impl Time {
    pub fn to_total_seconds(&self) -> u32 {
        self.hours as u32 * 3600 + self.minutes as u32 * 60 + self.seconds as u32
    }
}

pub fn bcd_to_dec(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0x0f)
}

pub fn dec_to_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_seconds() {
        let t = Time {
            hours: 1,
            minutes: 2,
            seconds: 3,
        };
        assert_eq!(t.to_total_seconds(), 3723);

        let t = Time {
            hours: 23,
            minutes: 59,
            seconds: 59,
        };
        assert_eq!(t.to_total_seconds(), 86399);
    }

    #[test]
    fn bcd_round_trip_over_the_rtc_range() {
        for dec in 0..=59u8 {
            assert_eq!(bcd_to_dec(dec_to_bcd(dec)), dec, "dec={dec}");
        }
    }

    #[test]
    fn bcd_encoding_is_packed_nibbles() {
        assert_eq!(dec_to_bcd(0), 0x00);
        assert_eq!(dec_to_bcd(9), 0x09);
        assert_eq!(dec_to_bcd(10), 0x10);
        assert_eq!(dec_to_bcd(59), 0x59);

        assert_eq!(bcd_to_dec(0x00), 0);
        assert_eq!(bcd_to_dec(0x23), 23);
        assert_eq!(bcd_to_dec(0x59), 59);
    }
}
