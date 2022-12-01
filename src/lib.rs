use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub type Paths = Vec<Box<PathBuf>>;

pub fn parse(files: Vec<String>) -> Result<Paths, &'static str> {
    if files.len() < 1 {
        return Err("not enough arguments");
    }

    let paths = files
        .into_iter()
        .map(|p| {
            let path = PathBuf::from(p);
            Box::new(path)
        })
        .collect();

    Ok(paths)
}

pub fn run(paths: Paths) -> Result<(), Box<dyn Error>> {
    for path in paths {
        let path = path.as_path();

        let display = path.display();

        if path.is_dir() {
            println!("rat: {0}: Is a directory", display);
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => format!("rat: Unable to read file {0}: {1}", display, err),
        };

        println!("{content}");
    }

    Ok(())
}

pub fn err(err: Box<dyn Error>) {
    println!("rat: application error: {0}", err)
}
