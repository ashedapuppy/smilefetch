use std::{error::Error, fmt};

use self::datalistbuilder::DataListBuilder;

mod datalistbuilder;
mod sysdata;

#[derive(Default)]
pub struct DataList {
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

impl DataList {
    pub fn custom(info_list: &str) -> Result<Self, Box<dyn Error>> {
        let mut data_list_builder = DataListBuilder::new();
        for c in info_list.chars() {
            data_list_builder = match c {
                'u' => data_list_builder.user(),
                'o' => data_list_builder.os(),
                'k' => data_list_builder.kernel(),
                't' => data_list_builder.uptime(),
                's' => data_list_builder.shell(),
                'c' => data_list_builder.cpu(),
                'm' => data_list_builder.mem(),
                'e' => data_list_builder.string("\n".to_string()),
                'r' => data_list_builder.string(sysdata::get_colors()),
                unknown => {
                    eprintln!("'{}' is not an available value (see --help)", unknown);
                    return Err("error parsing info list".into());
                }
            };
        }
        Ok(data_list_builder.build())
    }

    pub fn default() -> Self {
        DataListBuilder::new()
            .user()
            .string("\n".to_string())
            .os()
            .kernel()
            .uptime()
            .shell()
            .cpu()
            .mem()
            .string("\n".to_string())
            .string(sysdata::get_colors())
            .build()
    }
}
