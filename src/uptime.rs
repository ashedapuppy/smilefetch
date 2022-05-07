use std::fmt;
pub struct Uptime {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    total_seconds: i32,
}

impl Uptime {
    #[must_use]
    /// generates the Uptime struct using the total uptime (in seconds)
    /// It takes a float and returns a Time struct.
    ///
    /// Arguments:
    ///
    /// * `total_seconds`: The total amount of seconds that the timer will count down from.
    ///
    /// Returns:
    ///
    /// A new instance of the Time struct.
    pub fn new(total_seconds: f32) -> Self {
        let total_seconds_int = total_seconds as i32;
        let mut seconds = total_seconds;
        let days = seconds / (24f32 * 3600f32);
        seconds %= 24f32 * 3600f32;
        let hours = seconds / 3600f32;
        seconds %= 3600f32;
        let minutes = seconds / 60f32;

        let days = days as i32;
        let hours = hours as i32;
        let minutes = minutes as i32;
        let seconds = seconds as i32;
        Self {
            days,
            hours,
            minutes,
            seconds,
            total_seconds: total_seconds_int,
        }
    }
}

impl fmt::Display for Uptime {
    /// If the number of days, hours, and minutes are all zero, then print the number of seconds, otherwise
    /// print the number of days, hours, and minutes properly formatted to only include relevant values.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match (self.days, self.hours, self.minutes) {
                (0, 0, 0) => format!("{} seconds", self.total_seconds),
                (d, 0, 0) => format!("{} days", d),
                (0, h, 0) => format!("{} hours", h),
                (0, 0, m) => format!("{} minutes", m),
                (d, h, 0) => format!("{} days {} hours", d, h),
                (d, 0, m) => format!("{} days {} minutes", d, m),
                (0, h, m) => format!("{} hours {} minutes", h, m),
                (d, h, m) => format!("{} days {} hours {} minutes", d, h, m),
            }
        )
    }
}
