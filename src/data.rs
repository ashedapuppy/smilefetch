use std::{env, fmt, path, error::Error};

use colored::Colorize;
use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};

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

pub(crate) fn empty_line() -> &'static str {
    "\n"
}

pub(crate) fn get_os(sys: &mut System) -> Data<String> {
    Data::new("Os", sys.long_os_version().unwrap())
}

pub(crate) fn get_kernel(sys: &mut System) -> Data<String> {
    Data::new("Kernel", sys.kernel_version().unwrap())
}

pub(crate) fn get_cpuinfo(sys: &mut System) -> Data<String> {
    let cpuinfo = sys.global_processor_info();
    let cpu = format!("{} ({} MHz)", cpuinfo.brand(), cpuinfo.frequency());
    Data::new("Cpu", cpu)
}

pub(crate) fn get_uptime(sys: &mut System) -> Data<Uptime> {
    Data::new("Uptime", Uptime::new(sys.uptime()))
}

pub(crate) fn get_meminfo(sys: &mut System) -> Data<String> {
    let total = sys.total_memory() as f64;
    let used = sys.used_memory() as f64;
    let memstr = format!(
        "{} used out of {} ({:.2}%)",
        pretty_bytes::converter::convert(used * 1000f64),
        pretty_bytes::converter::convert(total * 1000f64),
        (used / total * 100f64)
    );
    Data::new("Memory", memstr)
}

fn get_shell_from_process(sys: &mut System) -> Result<String, Box<dyn Error>> {
    let current = match sys.process(sysinfo::get_current_pid()?) {
        Some(process) => process,
        None => Err("Couldn't get current process")?,
    };
    let parent = match sys.process(match current.parent() {
            Some(parent) => parent,
            None => Err("Couldn't get current process's parent")?,
        }) 
    {
        Some(process) => process,
        None => Err("no parent process")?
    };
    Ok(parent.name().to_string())
}

pub(crate) fn get_shell(sys: &mut System) -> Data<String> {
    let shell_env = match env::var("SHELL") {
        Ok(s) => Some(s),
        Err(_) => None,
    };
    let shell_name: String;
    if let Some(shell) = shell_env {
        let shell_tmp = path::Path::new(&shell).file_name().unwrap();
        shell_name = shell_tmp.to_str().unwrap().to_string();
    } else {
        shell_name = match get_shell_from_process(sys) {
            Ok(shell) => shell,
            Err(_) => String::from("shell not set")
        };
    }
    Data::new("Shell", shell_name)
}
