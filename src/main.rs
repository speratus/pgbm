extern crate clap;
use clap::{App, SubCommand};

use std::process::Command;

use std::io;

fn main() -> io::Result<()> {
    let matches = App::new("PostgreSQL booter")
        .version("1.0.2")
        .author("Andrew Luchuk")
        .about("Boots PostgreSQL as installed by Homebrew")
        .subcommand(SubCommand::with_name("start")
            .about("Starts PostgreSQL"))
        .subcommand(SubCommand::with_name("stop")
            .about("Stops PostgreSQL"))
        .subcommand(SubCommand::with_name("u")
            .about("Alias for \"start\" command. \"u\" stands for \"up\"."))
        .subcommand(SubCommand::with_name("d")
            .about("Alias for \"stop\". \"d\" stands for \"down\"."))
        .get_matches();

    match matches.subcommand_name() {
        Some("start") | Some("u") => boot()?,
        Some("stop") | Some("d") => shutdown()?,
        None | _ => println!("You must use either \"start\" or \"stop\" or one of their aliases."),
    }

    Ok(())
}

fn run_process(argument: &str) -> io::Result<()> {
    let args = vec!["-D", "/home/linuxbrew/.linuxbrew/var/postgresql@14", argument.clone()];
    let process = Command::new("pg_ctl")
        .args(args)
        .spawn()?;

    let output = process.wait_with_output()?;

    let stdout = match std::string::String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => panic!("Could not convert argument to utf-8"),
    };

    print!("{}", stdout);

    Ok(())
}

fn boot() -> io::Result<()>{
    run_process("start")?;

    Ok(())
}

fn shutdown() -> io::Result<()> {
    run_process("stop")?;

    Ok(())
}
