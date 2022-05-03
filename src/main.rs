use clap::Parser;

mod data;
use data::Data;

/// fast system info tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// clear screen before printing
    #[clap(short = 'c', long = "clear")]
    clear: bool,
}

fn main() {
    let args = Args::parse();
    let data = Data::new();
    if args.clear {
        if cfg!(unix) {
            std::process::Command::new("clear").status().unwrap();
        } else if cfg!(windows) {
            std::process::Command::new("cls").status().unwrap();
        }
    }
    println!("{}", data);
}
