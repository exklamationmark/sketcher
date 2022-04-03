// Note: this requires the `cargo` feature

use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("create a new article")
                .arg_required_else_help(true)
                .arg(arg!([CATEGORY]))
                .arg(arg!([TITLE]))
                .arg(arg!([TAGS])),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate static site from Pandoc markdown"),
        )
        .subcommand(
            Command::new("serve")
                .about("Serve static site")
                .arg_required_else_help(true)
                .arg(arg!([ADDRESS])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => println!("creating a new article with category= {:?}, title= {:?}, tags= {:?}", sub_matches.value_of("CATEGORY"),sub_matches.value_of("TITLE"), sub_matches.value_of("TAGS")),
        Some(("generate", _)) => println!("generate"),
        Some(("serve", sub_matches)) => println!("serving site from {:?}", sub_matches.value_of("ADDRESS")),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
