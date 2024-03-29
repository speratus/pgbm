extern crate clap;
use clap::{command, Command, arg};

use std::process;

use std::io;

fn main() -> io::Result<()> {
    let matches = command!()
        .version("1.1.0")
        .author("Andrew Luchuk")
        .about("Boots PostgreSQL as installed by Homebrew")
        .subcommand_required(true)
        .arg(
            arg!(
                -p --"postgres-version" <VERSION> "Chooses which version of Postgres to boot"
            )
                .required(false)
        )
        .arg(
            arg!(
                -d --"data-location" <LOCATION> "Sets the directory to use for storing database files"
            )
        )
        .subcommand(Command::new("start")
            .about("Starts PostgreSQL"))
        .subcommand(Command::new("stop")
            .about("Stops PostgreSQL"))
        .subcommand(Command::new("u")
            .about("Alias for \"start\" command. \"u\" stands for \"up\"."))
        .subcommand(Command::new("d")
            .about("Alias for \"stop\". \"d\" stands for \"down\"."))
        .get_matches();


    let v: &str;
    if let Some(version) = matches.get_one::<String>("postgres-version") {
        v = version.as_str();
    } else {
        v = "14";
    }

    let l: &str;
    if let Some(location) = matches.get_one::<String>("data-location") {
        l = location.as_str();
    } else {
        l = "/home/linuxbrew/.linuxbrew/var/postgresql";
    }

    match matches.subcommand_name() {
        Some("start") | Some("u") => boot(v, l)?,
        Some("stop") | Some("d") => shutdown(v, l)?,
        None | _ => println!("You must use either \"start\" or \"stop\" or one of their aliases."),
    }

    Ok(())
}

fn run_process(argument: &str, version: &str, location: &str) -> io::Result<()> {
    let path = format!("{}@{}", location.clone(), version.clone());
    let args = vec!["-D", path.as_str(), argument.clone()];
    let process = process::Command::new("pg_ctl")
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

fn boot(version: &str, locatinon: &str) -> io::Result<()>{
    run_process("start", version, locatinon)?;

    Ok(())
}

fn shutdown(version: &str, location: &str) -> io::Result<()> {
    run_process("stop", version, location)?;

    Ok(())
}
