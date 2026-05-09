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
