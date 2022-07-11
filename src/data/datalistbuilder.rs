use std::fmt;

use sysinfo::{System, SystemExt};

use super::sysdata;
use super::DataList;

#[derive(Default)]
pub struct DataListBuilder {
    list: Vec<Box<dyn fmt::Display>>,
    sysinfo: System,
}

impl DataListBuilder {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            sysinfo: System::new_all(),
        }
    }

    pub fn user(mut self) -> Self {
        let user = sysdata::get_user(&self.sysinfo);
        self.list.push(user);
        self
    }

    pub fn os(mut self) -> Self {
        let os = sysdata::get_os(&self.sysinfo);
        self.list.push(os);
        self
    }

    pub fn kernel(mut self) -> Self {
        let kernel = sysdata::get_kernel(&self.sysinfo);
        self.list.push(kernel);
        self
    }

    pub fn uptime(mut self) -> Self {
        let uptime = sysdata::get_uptime(&self.sysinfo);
        self.list.push(uptime);
        self
    }

    pub fn shell(mut self) -> Self {
        let shell = sysdata::get_shell();
        self.list.push(shell);
        self
    }

    pub fn cpu(mut self) -> Self {
        let cpu = sysdata::get_cpuinfo(&self.sysinfo);
        self.list.push(cpu);
        self
    }

    pub fn mem(mut self) -> Self {
        let mem = sysdata::get_meminfo(&self.sysinfo);
        self.list.push(mem);
        self
    }

    pub fn string(mut self, str: String) -> Self {
        self.list.push(Box::new(str));
        self
    }

    pub fn build(self) -> DataList {
        DataList { list: self.list }
    }
}
