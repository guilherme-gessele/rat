use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    program: String,
    paths: Vec<Box<PathBuf>>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let program = args[0].clone();

        let paths = args[1..]
            .into_iter()
            .map(|p| {
                let path = PathBuf::from(p);
                Box::new(path)
            })
            .collect();

        Ok(Config { program, paths })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    for path in &config.paths {
        let path = path.as_path();

        if path.is_dir() {
            println!("{0}: {1}: Is a directory", config.program, path.display());
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => format!(
                "{0}: Unable to read file {1}: {2}",
                config.program,
                path.display(),
                err
            ),
        };

        println!("{content}");
    }

    Ok(())
}

pub fn err(config: &Config, e: Box<dyn Error>) {
    println!("{0}: application error: {1}", config.program, e)
}
