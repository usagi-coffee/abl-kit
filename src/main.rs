use clap::{Arg, Command};

pub mod parser;
pub mod utils;

pub mod format;

fn cli() -> Command {
    Command::new("abl-kit")
        .about("ABL formatting tools")
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("fmt")
                .about("Reformat file (indents + style)")
                .arg(Arg::new("file")),
        )
        .subcommand(
            Command::new("fix")
                .about("Fixes file (fmt + defaults)")
                .arg(Arg::new("file")),
        )
        .subcommand(
            Command::new("extract")
                .about("Extracts JSDoc-like tag from file")
                .arg(Arg::new("file"))
                .arg(Arg::new("tag")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("fix", args)) => match args.get_one::<String>("file") {
            Some(file) => format::fix_file(file).expect("Failed to fix file"),
            None => panic!("File was not provided!"),
        },
        Some(("fmt", args)) => match args.get_one::<String>("file") {
            Some(file) => format::format_file(file).expect("Failed to format file"),
            None => panic!("File was not provided!"),
        },
        Some(("extract", args)) => match args.get_one::<String>("file") {
            Some(file) => {
                match utils::extract(
                    file,
                    args.get_one::<String>("tag")
                        .expect("Did not specify tag")
                        .clone(),
                ) {
                    Ok(text) => println!("{}", text.unwrap_or(String::new())),
                    Err(_) => println!(""),
                }
            }
            None => panic!("File was not provided!"),
        },
        _ => panic!("Unknown command"),
    }
}
