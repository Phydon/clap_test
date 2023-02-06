use clap::{arg, Command};

use std::process;

// TODO how to run only by typing in the name of the program ("uptest")
fn uptest() -> Command {
    Command::new("uptest")
        .about("Testing updates")
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com")
        // .allow_missing_positional(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new("run")
                .short_flag('r')
                // .long_flag("run")
                .about("run updates"),
        )
        .subcommand(
            Command::new("info")
                .about("print status informations")
                .short_flag('i')
                .arg(arg!(<APP> "The app to get information about"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("exclude")
                .about("exclude programs from update")
                .short_flag('e')
                .arg(arg!(<APP> "The app to exclude").num_args(1..))
                .arg_required_else_help(true),
        )
}

fn update() {
    println!("Updating...")
}

fn info(program: &str) {
    println!("Here`s some info about {}", program)
}

fn exclude(programs: String) {
    println!("These programs get excluded from the update: {}", programs)
}

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!("Received [ Ctrl-C ]! Quit program!");
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    let matches = uptest().get_matches();
    match matches.subcommand() {
        Some(("run", _)) => {
            update();
        }
        Some(("info", sub_matches)) => {
            info(sub_matches.get_one::<String>("APP").expect("required"));
        }
        Some(("exclude", sub_matches)) => {
            let apps: Vec<_> = sub_matches
                .get_many::<String>("APP")
                .expect("required")
                .map(|s| s.as_str())
                .collect();
            let programs = apps.join(", ");
            exclude(programs);
        }
        _ => unreachable!(),
    }
}
