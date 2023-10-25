use clap::{Arg, Command};

pub mod format;

fn cli() -> Command {
    Command::new("abl-kit")
        .about("ABL formatting tools")
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("fix")
                .about("Fixes indentations in the file(s)")
                .arg(Arg::new("file")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("fix", args)) => match args.get_one::<String>("file") {
            Some(file) => format::fix(file),
            None => println!("File was not provided!"),
        },
        _ => unreachable!(),
    }
}