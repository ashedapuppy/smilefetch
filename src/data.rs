use std::{env, fmt, fs, path};

use colored::Colorize;

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
    Data::new("os".to_string(), whoami::distro())
}

pub(crate) fn get_kernel() -> Data<String> {
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
    Data::new("kernel".to_string(), kernel)
}

pub(crate) fn get_uptime() -> Data<Uptime> {
    let file: String = fs::read_to_string("/proc/uptime").unwrap();
    let vec: Vec<&str> = file.split(' ').collect();

    let total_seconds = vec[0].to_string().parse().unwrap();
    Data::new("uptime".to_string(), Uptime::new(total_seconds))
}

pub(crate) fn get_shell() -> Data<String> {
    // TODO: refactor this to be more straightforward (try to remove OsStr maybe?)
    let shell_env = match env::var("SHELL") {
        Ok(s) => s,
        Err(_) => panic!("get_shell: shell not set"),
    };
    let shell_path = path::Path::new(&shell_env).file_name().unwrap();
    Data::new(
        "shell".to_string(),
        shell_path
            .to_str()
            .unwrap_or_else(|| panic!("get_shell: parsing error"))
            .to_string(),
    )
}
