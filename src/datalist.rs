use std::fmt;

use colored::Colorize;

use crate::data;

type Line<T> = Box<T>;

#[derive(Default)]
pub(crate) struct DataList {
    pub lines: Vec<Line<dyn fmt::Display>>,
}

impl fmt::Display for DataList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.lines.iter() {
            write!(f, "{}", line)?
        }
        Ok(())
    }
}

impl DataList {
    #[cfg(target_os = "linux")]
    pub(crate) fn new() -> Self {
        Self {
            lines: vec![
                Line::new(format!(
                    "{}@{}\n",
                    whoami::username().bold().blue(),
                    whoami::hostname().bold().blue()
                )),
                Line::new(data::empty_line()),
                Line::new(data::get_os()),
                Line::new(data::get_kernel()),
                Line::new(data::get_uptime()),
                Line::new(data::get_shell()),
                Line::new(data::empty_line()),
                Line::new(data::get_colors()),
            ],
        }
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn new() -> Self {
        Self {
            lines: vec![
                Line::new(format!(
                    "{}@{}\n",
                    whoami::username().bold().blue(),
                    whoami::hostname().bold().blue()
                )),
                Line::new(data::empty_line()),
                Line::new(data::get_colors()),
            ],
        }
    }
}
