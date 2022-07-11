use std::fmt;

use sysinfo::{System, SystemExt};

use crate::data;

#[derive(Default)]
pub(crate) struct DataListBuilder {
    list: Vec<Box<dyn fmt::Display>>,
    sysinfo: System,
}

#[derive(Default)]
pub(crate) struct DataList {
    list: Vec<Box<dyn fmt::Display>>,
}

impl fmt::Display for DataList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.list.iter() {
            write!(f, "{}", line)?
        }
        Ok(())
    }
}

impl DataListBuilder {
    pub(crate) fn new() -> Self {
        let sysinfo_mutex = System::new_all();
        Self {
            list: Vec::new(),
            sysinfo: sysinfo_mutex,
        }
    }

    pub(crate) fn user(mut self) -> Self {
        let user = data::get_user(&self.sysinfo);
        self.list.push(user);
        self
    }

    pub(crate) fn os(mut self) -> Self {
        let os = data::get_os(&self.sysinfo);
        self.list.push(os);
        self
    }

    pub(crate) fn kernel(mut self) -> Self {
        let kernel = data::get_kernel(&self.sysinfo);
        self.list.push(kernel);
        self
    }

    pub(crate) fn uptime(mut self) -> Self {
        let uptime = data::get_uptime(&self.sysinfo);
        self.list.push(uptime);
        self
    }

    pub(crate) fn shell(mut self) -> Self {
        let shell = data::get_shell();
        self.list.push(shell);
        self
    }

    pub(crate) fn cpu(mut self) -> Self {
        let cpu = data::get_cpuinfo(&self.sysinfo);
        self.list.push(cpu);
        self
    }

    pub(crate) fn mem(mut self) -> Self {
        let mem = data::get_meminfo(&self.sysinfo);
        self.list.push(mem);
        self
    }

    pub(crate) fn string(mut self, str: String) -> Self {
        self.list.push(Box::new(str));
        self
    }

    pub(crate) fn build(self) -> DataList {
        DataList { list: self.list }
    }
}

// TODO: build the datalist asynchronously
impl DataList {
    pub(crate) fn default() -> Self {
        DataListBuilder::new()
            .user()
            .os()
            .kernel()
            .uptime()
            .shell()
            .cpu()
            .mem()
            .string("\n\n".to_string())
            .string(data::get_colors())
            .build()
    }
}
