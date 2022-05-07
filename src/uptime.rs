use std::fmt;
pub(crate) struct Uptime {
    days: Unit,
    hours: Unit,
    minutes: Unit,
    seconds: Unit,
    // might be useful later idk
    _total_seconds: f64,
}

struct Unit {
    name: String,
    value: i32,
}

impl fmt::Display for Unit {
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
    pub(crate) fn new(total_seconds: f64) -> Self {
        let mut total_seconds_cp = total_seconds;
        let days = Unit {
            name: "day".to_string(),
            value: (total_seconds_cp / (24f64 * 3600f64)) as i32,
        };
        total_seconds_cp %= 24f64 * 3600f64;

        let hours = Unit {
            name: "hour".to_string(),
            value: (total_seconds_cp / 3600f64) as i32,
        };
        total_seconds_cp %= 3600f64;

        let minutes = Unit {
            name: "minute".to_string(),
            value: (total_seconds_cp / 60f64) as i32,
        };
        total_seconds_cp %= 60f64;

        let seconds = Unit {
            name: "second".to_string(),
            value: total_seconds_cp as i32,
        };

        Self {
            days,
            hours,
            minutes,
            seconds,
            _total_seconds: total_seconds,
        }
    }
}

impl fmt::Display for Uptime {
    /// If the number of days, hours, and minutes are all zero, then print the number of seconds, otherwise
    /// print the number of days, hours, and minutes properly formatted to only include relevant values.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!("{}{}{}{}", self.days, self.hours, self.minutes, self.seconds);
        write!(f, "{}", out.trim())
    }
}
