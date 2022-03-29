mod lib;

extern crate colored;

use colored::*;
use lib::Data;

fn main() {
    let data = Data::new();
    println!(
        r#"{}@{}

{}:		{}
{}:		{}
{}:		{}
{}:		{}

{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}"#,
        data.user.bold().blue(),
        data.hostname.bold().blue(),
        "Distro".bold().blue(),
        data.os.bold(),
        "Kernel".bold().blue(),
        data.kernel.bold(),
        "Uptime".bold().blue(),
        data.uptime.bold(),
        "Shell".bold().blue(),
        data.shell.bold(),
        "██".black(),
        "██".red(),
        "██".green(),
        "██".yellow(),
        "██".blue(),
        "██".magenta(),
        "██".cyan(),
        "██".white(),
        "██".bold().black(),
        "██".bold().red(),
        "██".bold().green(),
        "██".bold().yellow(),
        "██".bold().blue(),
        "██".bold().magenta(),
        "██".bold().cyan(),
        "██".bold().white(),
    );
}
