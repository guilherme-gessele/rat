use clap::{Args, Parser};
use rat::Paths;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    command: Command,

    files: Vec<String>,
}

#[derive(Args, Debug)]
struct Command {
    /// display $ at end of each line
    #[arg(short = 'E', long, default_value_t = false)]
    show_ends: bool,
}

fn main() {
    let cli = Cli::parse();

    let paths: Paths = match rat::parse(cli.files) {
        Ok(files) => files,
        Err(err) => {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(err) = rat::run(paths) {
        rat::err(err)
    }
}
