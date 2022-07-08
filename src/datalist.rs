use std::fmt;

use colored::Colorize;
use sysinfo::{System, SystemExt};

use crate::data;

#[derive(Default)]
pub(crate) struct DataList(Vec<Box<dyn fmt::Display>>);

impl fmt::Display for DataList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.0.iter() {
            write!(f, "{}", line)?
        }
        Ok(())
    }
}

impl DataList {
    pub(crate) fn default() -> Self {
        let mut sysinfo = System::new_all();
        Self(vec![
            Box::new(format!(
                "{}@{}\n",
                whoami::username().bold().blue(),
                sysinfo
                    .host_name()
                    .unwrap_or_else(|| "{error}".to_string())
                    .bold()
                    .blue()
            )),
            Box::new("\n"),
            data::get_os(&mut sysinfo),
            data::get_kernel(&mut sysinfo),
            data::get_uptime(&mut sysinfo),
            data::get_shell(),
            data::get_cpuinfo(&mut sysinfo),
            data::get_meminfo(&mut sysinfo),
            Box::new("\n"),
            data::get_colors(),
        ])
    }
}
