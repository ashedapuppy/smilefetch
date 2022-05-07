use std::{env, fmt, path};

use proc_getter::cpuinfo::cpuinfo;
use proc_getter::meminfo::meminfo;
use proc_getter::uptime::uptime;
use proc_getter::version::version;
use pretty_bytes::converter::convert;
use colored::Colorize;
use sys_info;

use crate::uptime::Uptime;

pub(crate) struct Data<T> {
    name: String,
    value: T,
}

impl<T> Data<T> {
    fn new(name: String, value: T) -> Self {
        Self { name, value }
    }
}

impl<T> std::fmt::Display for Data<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\t\t{}\n", self.name.bold().blue(), self.value)
    }
}

pub(crate) fn get_colors() -> String {
    let box_char = "██";
    format!(
        "{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}",
        box_char.black(),
        box_char.red(),
        box_char.green(),
        box_char.yellow(),
        box_char.blue(),
        box_char.magenta(),
        box_char.cyan(),
        box_char.white(),
        box_char.bold().black(),
        box_char.bold().red(),
        box_char.bold().green(),
        box_char.bold().yellow(),
        box_char.bold().blue(),
        box_char.bold().magenta(),
        box_char.bold().cyan(),
        box_char.bold().white(),
    )
}

pub(crate) fn empty_line() -> &'static str {
    "\n"
}

pub(crate) fn get_os() -> Data<String> {
    let release = sys_info::linux_os_release().unwrap();
    Data::new(
        "Os".to_string(),
        release.pretty_name.unwrap_or_else(|| release.name.unwrap()),
    )
}

pub(crate) fn get_kernel() -> Data<String> {
    let mut kernel: String = String::new();
    let file: String = version().unwrap();
    for line in file.lines() {
        let vec: Vec<&str> = line.split(' ').collect();
        for (i, _) in vec.iter().enumerate().take(3) {
            kernel.push_str(vec[i]);
            if i != 2 {
                kernel.push(' ')
            }
        }
    }
    Data::new("Kernel".to_string(), kernel)
}

pub(crate) fn get_cpuinfo() -> Data<String> {
    let cpuinfo = cpuinfo().unwrap();
    let cpu = format!("{} ({} threads)", cpuinfo[0].model_name(), cpuinfo.len());
    Data::new("Cpu".to_string(), cpu)
}

pub(crate) fn get_uptime() -> Data<Uptime> {
    let uptime = uptime().unwrap();
    Data::new("Uptime".to_string(), Uptime::new(*uptime.total()))
}

pub(crate) fn get_meminfo() -> Data<String> {
    let info = meminfo().unwrap();
    // multiply by 1000 because we get a value in kB and convert takes a value in bytes
    let total: usize = *info.get("MemTotal").unwrap() * 1000;
    let used = total - info.get("MemAvailable").unwrap() * 1000;
    let memstr = format!("{} / {} ({}%)", 
        convert(used as f64),
        convert(total as f64),
        (used / total * 100) 
    );
    Data::new("Memory".to_string(), memstr)
}

pub(crate) fn get_shell() -> Data<String> {
    // TODO: refactor this to be more straightforward (try to remove OsStr maybe?)
    let shell_env = match env::var("SHELL") {
        Ok(s) => s,
        Err(_) => panic!("get_shell: shell not set"),
    };
    let shell_path = path::Path::new(&shell_env).file_name().unwrap();
    Data::new(
        "Shell".to_string(),
        shell_path
            .to_str()
            .unwrap_or_else(|| panic!("get_shell: parsing error"))
            .to_string(),
    )
}
