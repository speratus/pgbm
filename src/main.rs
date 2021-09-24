extern crate clap;
use clap::{App, SubCommand};

use std::process::Command;

use std::io;

fn main() -> io::Result<()> {
    let matches = App::new("PostgreSQL booter")
        .version("1.0.0")
        .author("Andrew Luchuk")
        .about("Boots PostgreSQL as installed by Homebrew")
        .subcommand(SubCommand::with_name("start")
            .about("Starts PostgreSQL"))
        .subcommand(SubCommand::with_name("stop")
            .about("Stops PostgreSQL"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("start") {
        boot()?;
        return Ok(());
    }

    if let Some(_matches) = matches.subcommand_matches("stop") {
        shutdown()?;
        return Ok(());
    }

    println!("You must use either \"start\" or \"stop\".");

    Ok(())
}

fn run_process(argument: &str) -> io::Result<()> {
    let args = vec![argument.clone()];
    let process = Command::new("/home/linuxbrew/.linuxbrew/var/postgres")
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
    run_process("shutdown")?;

    Ok(())
}
