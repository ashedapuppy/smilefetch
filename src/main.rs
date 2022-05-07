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
}

fn clear_term() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    std::process::Command::new("clear").status().unwrap();

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status()
        .unwrap();
}

fn main() {
    let args = Args::parse();
    let data = DataList::default();
    if args.clear {
        clear_term();
    }
    println!("{}", data);
}
