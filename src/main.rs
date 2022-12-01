use clap::Parser;
use rat::cli::Cli;
use rat::Paths;
use std::process;

fn main() {
    let cli = Cli::parse();

    let paths: Paths = match rat::parse(cli.files) {
        Ok(files) => files,
        Err(err) => {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(err) = rat::run(paths, cli.command) {
        rat::err(err)
    }
}
