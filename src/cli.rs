use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub command: Command,

    pub files: Vec<String>,
}

#[derive(Args, Debug)]
pub struct Command {
    /// display $ at end of each line
    #[arg(short = 'E', long, default_value_t = false)]
    pub show_ends: bool,
}
