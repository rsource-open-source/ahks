//use colored::Colorize;
use std::{
    io::{stdin, stdout, Read, Write},
    process::Command,
};

use colored::Colorize;

pub fn exec(string: &str, func: &dyn Fn() -> Result<&'static str, &'static str>) {
    println!("\n{}...", string);
    match func() {
        Ok(s) => println!("{} {}", "ok".green(), s),
        Err(s) => println!("{} {}", "error".red(), s),
    }
    println!("")
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    std::process::exit(0);
}

pub fn open(path: &str) {
    Command::new("explorer")
        .arg(path)
        .spawn()
        .expect("Failed to open explorer");
}
