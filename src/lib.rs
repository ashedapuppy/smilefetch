use std::{env, fs, path};

#[allow(dead_code)]
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
    pub fn print(&self) {
        println!(
            "os={}\nkernel={}\nuptime={}\nhostname={}\nshell={}\nuser={}\ncpuinfo={}\nmeminfo={}",
            self.os,
            self.kernel,
            self.uptime,
            self.hostname,
            self.shell,
            self.user,
            self.cpuinfo,
            self.meminfo
        )
    }

    pub fn new() -> Self {
        Data {
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

        let mut seconds: f64 = vec[0].to_string().parse().unwrap();
        let days = seconds / (24f64 * 3600f64);
        seconds %= 24f64 * 3600f64;
        let hours = seconds / 3600f64;
        seconds %= 3600f64;
        let minutes = seconds / 60f64;

        let days = days as i64;
        let hours = hours as i64;
        let minutes = minutes as i64;
        let seconds = seconds as i64;

        let uptime: String = match (days, hours, minutes) {
            (0i64, 0i64, 0i64) => format!("{} seconds", seconds),
            (0i64, 0i64, m) => format!("{} minutes", m),
            (0i64, h, 0i64) => format!("{} hours", h),
            (0i64, h, m) => format!("{} hours {} minutes", h, m),
            (d, 0i64, 0i64) => format!("{} days", d),
            (d, 0i64, m) => format!("{} days {} minutes", d, m),
            (d, h, 0i64) => format!("{} days {} hours", d, h),
            (d, h, m) => format!("{} days {} hours {} minutes", d, h, m),
        };
        uptime
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
