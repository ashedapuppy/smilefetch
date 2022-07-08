use std::{fmt, sync::Arc};

use sysinfo::{System, SystemExt};
use tokio::sync::Mutex;

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

// TODO: build the datalist asynchronously
impl DataList {
    pub(crate) async fn default() -> Self {
        let sysinfo_mutex = Arc::new(Mutex::new(System::new_all()));
        let (user, os, kernel, uptime, shell, cpu, mem) = tokio::join!(
            data::get_user(&sysinfo_mutex),
            data::get_os(&sysinfo_mutex),
            data::get_kernel(&sysinfo_mutex),
            data::get_uptime(&sysinfo_mutex),
            data::get_shell(),
            data::get_cpuinfo(&sysinfo_mutex),
            data::get_meminfo(&sysinfo_mutex),
        );
        Self(vec![
            user,
            os,
            kernel,
            uptime,
            shell,
            cpu,
            mem,
            Box::new("\n"),
            Box::new("\n"),
            data::get_colors(),
        ])
    }
}
