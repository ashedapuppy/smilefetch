use std::error::Error;

use clap::Parser;

mod data;
mod datalist;
mod uptime;

use datalist::DataList;

/// fast system info tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// clear screen before printing
    #[clap(short = 'c', long = "clear")]
    clear: bool,

    /// build the info in this order
    ///
    /// possible info :
    /// [user=u], [os=o], [kernel=k], [uptime=t], [shell=s], [cpu=c], [mem=m], [empty_line=e], [rainbow=r]
    #[clap(short = 'i', long = "info")]
    info: Option<String>,
}

fn verify_info(info_list: &str) -> Result<(), Box<dyn Error>> {
    let allowed = ['u', 'o', 'k', 't', 's', 'c', 'm', 'e', 'r'];
    for c in info_list.chars() {
        if !allowed.contains(&c) {
            eprintln!("'{}' is not an available info (see --help)", &c);
            return Err("error parsing info list".into());
        }
    }
    Ok(())
}

fn clear_term() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    std::process::Command::new("clear")
        .status()
        .expect("could not clear terminal");

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status()
        .expect("could not clear terminal");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // if user gave custom info list, build it, otherwise use the default
    let data = if let Some(info_list) = args.info {
        verify_info(&info_list)?;
        DataList::custom(&info_list)?
    } else {
        DataList::default()
    };
    if args.clear {
        clear_term();
    }
    println!("{}", data);
    Ok(())
}
