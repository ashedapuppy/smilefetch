mod lib;

use lib::Data;

fn main() {
    let data = Data::new();
    if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    } else if cfg!(windows) {
        std::process::Command::new("cls").status().unwrap();
    }
    println!("{}", data);
}
