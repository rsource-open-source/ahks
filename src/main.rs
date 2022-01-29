// use std::{fs, io::stdin, path::Path, time::Duration};

// mod config;
mod config;
mod fns;
use colored::control;
use std::{fs::File, path::Path};

use crate::{
    config::find_file,
    fns::{await_input, exec, open, pause},
};

/*
INTRO
load config file
if no config file, create one
ask for ahk directory, if none, ask if make ahk directory
put ahk directory into config file

COMMAND:
    list                - list all .ahk files in ahk directory
    open <ahk?>         - open ahk directory in explorer
    setbind <ahk>       - set a new binding for an ahk file
    chatbind <name>     - new ahk for the chat
    transpile <ahk>     - will transpile an ahk file into a raw copy/pasteable version
    run <ahk>           - run an ahk file

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

    if !Path::new("./config.toml").exists() {
        // New user
        println!("Hi! We're just setting some stuff up before you start...");
        exec("Create config file", &|| {
            // let mut file = match File::create("./config.toml") {
            //     Ok(file) => "file created",
            //     Err(e) => e.to_string(),
            // };

            let f = File::create("./config.toml");
            if f.is_ok() {
                Ok("file created")
            } else {
                Err("some error") // implement actual error
            }
        });
    }
    {
        // find ahk exec
        if find_file("C:\\Program Files\\AutoHotkey\\AutoHotkey.exe").is_err() {
            println!("AutoHotkey.exe not found. Why did you move it? :|");
            pause(); // TODO: ask where it is AND implement verification
        }
    }

    exec("2222", &|| {
        if 1 + 1 == 3 {
            Ok("1")
        } else {
            Err("1+1 != 3")
        }
    });

    open(".");
    await_input();
    pause();
}
