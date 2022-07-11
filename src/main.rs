use std::error::Error;

use clap::Parser;

mod data;
mod uptime;

use data::DataList;

/// fast system info tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// clear screen before printing
    #[clap(short = 'c', long = "clear")]
    clear: bool,

    /// build the infolist with a string
    ///
    /// possible info :
    ///     [user=u]    [os=o]    [kernel=k]  [uptime=t],
    ///     [shell=s]   [cpu=c]   [mem=m]     [empty_line=e],
    ///     [rainbow=r]
    #[clap(verbatim_doc_comment)]
    #[clap(short = 'i', long = "info")]
    info: Option<String>,
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
    // if user gave custom info list, build it,
    // otherwise use the default
    let data = if let Some(info_list) = args.info {
        data::verify_infolist(&info_list)?;
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
