// use std::{fs, io::stdin, path::Path, time::Duration};

// mod config;
mod config;
mod fns;
use colored::control;

use crate::{
    config::find_ahk_exe,
    fns::{await_input, pause},
};

/*
INTRO
load config file
if no config file, create one
ask for ahk directory, if none, ask if make ahk directory
put ahk directory into config file

COMMAND:
    list - list all .ahk files in ahk directory
    open - open ahk directory in explorer
    setbind <ahkfile> - set a binding for an ahk file
    unsetbind <ahkfile> - unset a binding for an ahk file
    doubleslash - if true, double slash
    transpile - will transpile an ahk file into a raw copy/pasteable version

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
    {
        // find ahk exec
        if find_ahk_exe().is_err() {
            println!("AutoHotkey.exe not found. Why did you move it? :|");
            pause(); // TODO: ask where it is AND implement verification
        }
    }

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
