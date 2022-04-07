use colored::*;
use std::{env, fmt, fs, path};

pub struct Uptime {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    total_seconds: i32,
}

impl Uptime {
    /// generates the Uptime struct using the total uptime (in seconds)
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

#[derive(Default)]
pub struct Data {
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub hostname: String,
    pub shell: String,
    pub user: String,
}

// due to the formatting of the files we get the information from, I decided
// it would be easier to maintain if there was a specific function for each
// value, it also incidentally makes it easy to expand this to work with any
// system by using conditional compilation
impl fmt::Display for Data {
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
            "████".black(),
            "████".red(),
            "████".green(),
            "████".yellow(),
            "████".blue(),
            "████".magenta(),
            "████".cyan(),
            "████".white(),
            "████".bright_black(),
            "████".bright_red(),
            "████".bright_green(),
            "████".bright_yellow(),
            "████".bright_blue(),
            "████".bright_magenta(),
            "████".bright_cyan(),
            "████".bright_white(),
        )
    }
}

// only works on linux (for now)
#[cfg(target_os = "linux")]
impl Data {
    pub fn new() -> Self {
        Self {
            os: Self::get_os(),
            kernel: Self::get_kernel(),
            uptime: Self::get_uptime(),
            hostname: Self::get_hostname(),
            shell: Self::get_shell(),
            user: Self::get_user(),
        }
    }

    fn get_os() -> String {
        whoami::distro()
    }

    fn get_kernel() -> String {
        let mut kernel: String = String::new();
        let file: String = fs::read_to_string("/proc/version").unwrap();
        for line in file.lines() {
            let vec: Vec<&str> = line.split(' ').collect();
            for (i, _) in vec.iter().enumerate().take(3) {
                kernel.push_str(vec[i]);
                if i != 2 {
                    kernel.push(' ')
                }
            }
        }
        kernel
    }

    fn get_uptime() -> String {
        let file: String = fs::read_to_string("/proc/uptime").unwrap();
        let vec: Vec<&str> = file.split(' ').collect();

        let total_seconds = vec[0].to_string().parse().unwrap();
        Uptime::new(total_seconds).to_string()
    }

    fn get_hostname() -> String {
        whoami::hostname()
    }

    fn get_shell() -> String {
        let shell_str = match env::var("SHELL") {
            Ok(s) => s,
            Err(_) => panic!("shell not set"),
        };
        let shell_path = path::Path::new(&shell_str).file_name().unwrap();
        shell_path.to_str().unwrap().to_string()
    }

    fn get_user() -> String {
        whoami::username()
    }
}

// TODO: add data constructor for windows
#[cfg(not(target_os = "linux"))]
impl Data {
    pub fn new() -> Self {
        Self {
            os: String::from(""),
            kernel: String::from(""),
            uptime: String::from(""),
            hostname: String::from(""),
            shell: String::from(""),
            user: String::from(""),
        }
    }
}
