use std::fmt;

use colored::Colorize;

use crate::data;

impl fmt::Display for data::Uptime {
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

impl fmt::Display for data::Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{}@{}

{}:		{}
{}:		{}
{}:		{}
{}:		{}

{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}"#,
            self.user.bold().blue(),
            self.hostname.bold().blue(),
            "Distro".bold().blue(),
            self.os.bold(),
            "Kernel".bold().blue(),
            self.kernel.bold(),
            "Uptime".bold().blue(),
            self.uptime.bold(),
            "Shell".bold().blue(),
            self.shell.bold(),
            "██".black(),
            "██".red(),
            "██".green(),
            "██".yellow(),
            "██".blue(),
            "██".magenta(),
            "██".cyan(),
            "██".white(),
            "██".bold().black(),
            "██".bold().red(),
            "██".bold().green(),
            "██".bold().yellow(),
            "██".bold().blue(),
            "██".bold().magenta(),
            "██".bold().cyan(),
            "██".bold().white(),
        )
    }
}
