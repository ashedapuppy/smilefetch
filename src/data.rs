use std::fmt;

use colored::Colorize;
use etc_passwd::Passwd;
use sysinfo::{CpuExt, System, SystemExt};

use crate::uptime::Uptime;

pub(crate) struct Data<T> {
    name: &'static str,
    value: T,
}

impl<T> Data<T> {
    fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

impl<T> fmt::Display for Data<T>
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

pub(crate) fn get_os(sys: &System) -> Box<dyn fmt::Display> {
    match sys.long_os_version() {
        Some(os) => Box::new(Data::new("Os", os)),
        None => Box::new(""),
    }
}

pub(crate) fn get_kernel(sys: &System) -> Box<dyn fmt::Display> {
    match sys.kernel_version() {
        Some(kernel) => Box::new(Data::new("Kernel", kernel)),
        None => Box::new(""),
    }
}

pub(crate) fn get_cpuinfo(sys: &System) -> Box<dyn fmt::Display> {
    let cpuinfo = sys.cpus();
    let cpu = format!("{} ({} MHz)", cpuinfo[0].brand(), cpuinfo[0].frequency());
    Box::new(Data::new("Cpu", cpu))
}

pub(crate) fn get_uptime(sys: &System) -> Box<dyn fmt::Display> {
    Box::new(Data::new("Uptime", Uptime::new(sys.uptime())))
}

pub(crate) fn get_meminfo(sys: &System) -> Box<dyn fmt::Display> {
    let total = sys.total_memory() as f64;
    let used = sys.used_memory() as f64;
    let memstr = format!(
        "{} used out of {} ({:.2}%)",
        pretty_bytes::converter::convert(used * 1000f64),
        pretty_bytes::converter::convert(total * 1000f64),
        (used / total * 100f64)
    );
    Box::new(Data::new("Memory", memstr))
}

pub(crate) fn get_shell() -> Box<dyn fmt::Display> {
    let shell: Option<String> = match Passwd::current_user() {
        Ok(Some(p)) => p.shell.to_str().map(|s| s.to_string()).ok(),
        _ => None,
    };
    match shell {
        Some(shell) => Box::new(Data::new("Shell", shell)),
        None => Box::new(""),
    }
}

pub(crate) fn get_user(sys: &System) -> Box<dyn fmt::Display> {
    Box::new(format!(
        "{}@{}\n",
        whoami::username().bold().blue(),
        sys.host_name()
            .unwrap_or_else(|| "{error}".to_string())
            .bold()
            .blue()
    ))
}
