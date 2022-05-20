use std::fmt;

use colored::Colorize;
use sysinfo::{System, SystemExt};

use crate::data;

type Line<T> = Box<T>;

#[derive(Default)]
pub(crate) struct DataList(Vec<Line<dyn fmt::Display>>);

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
            Line::new(format!(
                "{}@{}\n",
                whoami::username().bold().blue(),
                sysinfo.host_name().unwrap().bold().blue()
            )),
            Line::new(data::empty_line()),
            Line::new(data::get_os(&mut sysinfo)),
            Line::new(data::get_kernel(&mut sysinfo)),
            Line::new(data::get_uptime(&mut sysinfo)),
            Line::new(data::get_shell(&mut sysinfo)),
            Line::new(data::get_cpuinfo(&mut sysinfo)),
            Line::new(data::get_meminfo(&mut sysinfo)),
            Line::new(data::empty_line()),
            Line::new(data::get_colors()),
        ])
    }
}
