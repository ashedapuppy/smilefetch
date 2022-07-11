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
        let mut builder = DataListBuilder::new();
        for c in info_list.chars() {
            builder = match c {
                'u' => builder.user(),
                'o' => builder.os(),
                'k' => builder.kernel(),
                't' => builder.uptime(),
                's' => builder.shell(),
                'c' => builder.cpu(),
                'm' => builder.mem(),
                'e' => builder.string("\n".to_string()),
                'r' => builder.string(sysdata::get_colors()),
                _ => return Err("unrecognised option (info list verification failed)".into()),
            };
        }
        Ok(builder.build())
    }

    pub fn default() -> Result<Self, Box<dyn Error>> {
        Ok(DataListBuilder::new()
            .user()
            .os()
            .kernel()
            .uptime()
            .shell()
            .cpu()
            .mem()
            .string("\n\n".to_string())
            .string(sysdata::get_colors())
            .build())
    }
}

pub fn verify_infolist(info_list: &str) -> Result<(), Box<dyn Error>> {
    let allowed = ['u', 'o', 'k', 't', 's', 'c', 'm', 'e', 'r'];
    for c in info_list.chars() {
        if !allowed.contains(&c) {
            eprintln!("'{}' is not an available info (see --help)", &c);
            return Err("error parsing info list".into());
        }
    }
    Ok(())
}
