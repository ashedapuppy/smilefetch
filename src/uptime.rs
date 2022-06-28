use std::fmt;

struct UUnit {
    name_singular: &'static str,
    value: u16,
}

pub(crate) struct Uptime {
    days: UUnit,
    hours: UUnit,
    minutes: UUnit,
    seconds: UUnit,
}

impl fmt::Display for UUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.value {
                0 => String::new(),
                1 => format!("1 {} ", &self.name_singular),
                n => format!("{n} {}s ", &self.name_singular),
            }
        )
    }
}

impl UUnit {
    fn days(value: u16) -> Self {
        Self {
            name_singular: "day",
            value
        }
    }
    fn hours(value: u16) -> Self {
        Self {
            name_singular: "hour",
            value
        }
    }
    fn minutes(value: u16) -> Self {
        Self {
            name_singular: "minute",
            value
        }
    }
    fn seconds(value: u16) -> Self {
        Self {
            name_singular: "second",
            value
        }
    }
}

impl Uptime {
    pub(crate) fn new(total_seconds: u64) -> Self {
        let mut total_seconds = total_seconds;

        let days = UUnit::days((total_seconds / (24 * 3600)) as u16);
        total_seconds %= 24 * 3600;

        let hours = UUnit::hours((total_seconds / 3600) as u16);
        total_seconds %= 3600;
        
        let minutes = UUnit::minutes((total_seconds / 60) as u16);
        total_seconds %= 60;

        let seconds = UUnit::seconds(total_seconds as u16);

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
