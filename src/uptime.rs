use std::fmt;
pub(crate) struct Uptime {
    days: UUnit,
    hours: UUnit,
    minutes: UUnit,
    seconds: UUnit,
}

struct UUnit {
    name: &'static str,
    value: u16,
}

impl fmt::Display for UUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.value {
                0 => String::new(),
                1 => format!("1 {} ", &self.name),
                n => format!("{n} {}s ", &self.name),
            }
        )
    }
}

impl Uptime {
    pub(crate) fn new(total_seconds: u64) -> Self {
        let mut total_seconds = total_seconds;
        let days = UUnit {
            name: "day",
            value: (total_seconds / (24 * 3600)) as u16,
        };
        total_seconds %= 24 * 3600;

        let hours = UUnit {
            name: "hour",
            value: (total_seconds / 3600) as u16,
        };
        total_seconds %= 3600;

        let minutes = UUnit {
            name: "minute",
            value: (total_seconds / 60) as u16,
        };
        total_seconds %= 60;

        let seconds = UUnit {
            name: "second",
            value: total_seconds as u16,
        };

        Self {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for Uptime {
    /// If the number of days, hours, and minutes are all zero, then print the number of seconds, otherwise
    /// print the number of days, hours, and minutes properly formatted to only include relevant values.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "{}{}{}{}",
            self.days, self.hours, self.minutes, self.seconds
        );
        write!(f, "{}", out.trim())
    }
}
