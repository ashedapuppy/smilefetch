use std::{env, fs, path};

struct Uptime {
    days: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
}

impl Uptime {
    #[must_use]
    /// generates the Uptime struct using the total uptime (in seconds)
    fn new(total_seconds: f32) -> Self {
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
        }
    }
}

#[derive(Debug, Default)]
pub struct Data {
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub hostname: String,
    pub shell: String,
    pub user: String,
    pub cpuinfo: String,
    pub meminfo: String,
}

#[allow(dead_code)]
#[cfg(all(unix))]
#[cfg(not(target_vendor = "apple"))]
// due to the formatting of the files we get the information from, I decided
// it would be easier to maintain if there was a specific function for each
// value, it also incidentally makes it easy to expand this to work with any
// system by using conditional compilation
impl Data {
    #[must_use]
    pub fn new() -> Self {
        Self {
            os: Data::get_os(),
            kernel: Data::get_kernel(),
            uptime: Data::get_uptime(),
            hostname: Data::get_hostname(),
            shell: Data::get_shell(),
            user: Data::get_user(),
            cpuinfo: String::from(""),
            meminfo: String::from(""),
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
        let uptime = Uptime::new(total_seconds);
        match (uptime.days, uptime.hours, uptime.minutes) {
            (0, 0, 0) => format!("{} seconds", uptime.seconds),
            (d, 0, 0) => format!("{} days", d),
            (0, h, 0) => format!("{} hours", h),
            (0, 0, m) => format!("{} minutes", m),
            (d, h, 0) => format!("{} days {} hours", d, h),
            (d, 0, m) => format!("{} days {} minutes", d, m),
            (0, h, m) => format!("{} hours {} minutes", h, m),
            (d, h, m) => format!("{} days {} hours {} minutes", d, h, m),
        }
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
