use clap::{App, Arg, SubCommand};
use std::process::Command;

fn clone(repos: Vec<&str>, dest: &str) -> () {
    for repo in repos {
        println!("Cloning {} to {}", repo, dest);
        let mut output = Command::new("git")
            .current_dir(dest)
            .args(&["clone", repo])
            .spawn().ok().expect("Failed to clone repo");
        output.wait().expect("An error occured in spawned git process");
    }
}

fn main() {
    let matches = App::new("gitalot")
        .version("v0.1.0")
        .author("Ethan Evans <ethanalexevans@gmail.com>")
        .about("A tool for interacting with more than one git repository at a time")
        .subcommand(
            SubCommand::with_name("clone")
                .arg(
                    Arg::with_name("listfile")
                        .required(true)
                        .takes_value(true)
                        .help("List of repositories to clone"),
                )
                .arg(
                    Arg::with_name("destination")
                        .required(true)
                        .takes_value(true)
                        .help("Destination directory"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("clone") => {
            let list = matches.subcommand_matches("clone").unwrap().value_of("listfile").unwrap();
            let dest = matches.subcommand_matches("clone").unwrap().value_of("destination").unwrap();

            let contents = std::fs::read_to_string(list).unwrap().replace("\r\n", "\n");
            let repos = contents.split("\n").collect::<Vec<&str>>();
            clone(repos, dest);
        },
        _ => {
            println!("No subcommand provided. use --help for more information");
        }
    }
}
