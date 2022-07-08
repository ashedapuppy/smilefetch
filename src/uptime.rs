use std::fmt;

struct TimeUnit {
    name: &'static str,
    value: u16,
}

pub(crate) struct Uptime {
    days: TimeUnit,
    hours: TimeUnit,
    minutes: TimeUnit,
    seconds: TimeUnit,
}

impl fmt::Display for TimeUnit {
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

impl TimeUnit {
    fn days(value: u16) -> Self {
        Self { name: "day", value }
    }
    fn hours(value: u16) -> Self {
        Self {
            name: "hour",
            value,
        }
    }
    fn minutes(value: u16) -> Self {
        Self {
            name: "minute",
            value,
        }
    }
    fn seconds(value: u16) -> Self {
        Self {
            name: "second",
            value,
        }
    }
}

impl Uptime {
    pub(crate) fn new(total_seconds: u64) -> Self {
        let mut total_seconds = total_seconds;

        let days = TimeUnit::days((total_seconds / (24 * 3600)) as u16);
        total_seconds %= 24 * 3600;

        let hours = TimeUnit::hours((total_seconds / 3600) as u16);
        total_seconds %= 3600;

        let minutes = TimeUnit::minutes((total_seconds / 60) as u16);
        total_seconds %= 60;

        let seconds = TimeUnit::seconds(total_seconds as u16);

        Self {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            "{}{}{}{}",
            self.days, self.hours, self.minutes, self.seconds
        );
        write!(f, "{}", out.trim())
    }
}
