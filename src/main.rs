// use std::{fs, io::stdin, path::Path, time::Duration};

// mod config;
mod fns;
use colored::control;

use crate::fns::await_input;

/*
terminal, manage ahks and their initial bindings

INTRO
load config file
if no config file, create one
ask for ahk directory
put ahk directory into config file

COMMAND:
    list - list all .ahk files in ahk directory
    setbind <ahkfile> - set a binding for an ahk file
    unsetbind <ahkfile> - unset a binding for an ahk file
    doubleslash - if true, double slash
    rusure - if true, will prompt "are you sure?""

BINDING MANAGEMENT

list all .ahk files in ahk directory

INFO:
    all the ahks are modular

https://github.com/redox-os/termion
*/

fn main() {
    control::set_virtual_terminal(true).unwrap();
    if cfg!(not(target_os = "windows")) {
        println!("{}", "AutoHotkey is only supported on Windows.");
        fns::pause();
    }

    println!("Hi! We're just setting some stuff up before you start...");
    // fns::exec("Finding Config File", &|| {});

    fns::exec("2222", &|| {
        if 1 + 1 == 3 {
            Ok("1")
        } else {
            Err("1+1 != 3")
        }
    });

    fns::open(".");
    await_input();
    fns::pause();
}
