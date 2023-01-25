extern crate clap;
use clap::{command, Command, arg, value_parser};

use std::process;

use std::io;

fn main() -> io::Result<()> {
    let matches = command!()
        .version("1.0.2")
        .author("Andrew Luchuk")
        .about("Boots PostgreSQL as installed by Homebrew")
        .subcommand_required(true)
        .arg(
            arg!(
                -p --postgres-version <VERSION> "Chooses which version of Postgres to boot"
            )
                .required(false)
                .value_parser(value_parser!(String)),
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
    if let Some(version) = matches.get_one::<String>("version") {
        v = version.as_str();
    } else {
        v = "14";
    }

    match matches.subcommand_name() {
        Some("start") | Some("u") => boot(v)?,
        Some("stop") | Some("d") => shutdown(v)?,
        None | _ => println!("You must use either \"start\" or \"stop\" or one of their aliases."),
    }

    Ok(())
}

fn run_process(argument: &str, version: &str) -> io::Result<()> {
    let path = format!("/home/linuxbrew/.linuxbrew/var/postgresql@{}", version.clone());
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

fn boot(version: &str) -> io::Result<()>{
    run_process("start", version)?;

    Ok(())
}

fn shutdown(version: &str) -> io::Result<()> {
    run_process("stop", version)?;

    Ok(())
}
