use std::{
    path,
    env
};

#[derive(Debug)]
struct Config {
    pub destination: Box<path::Path>,
    pub no_ms: bool
} impl Config {
    pub fn from_args() -> Result<Config, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            return Err("Not enough arguments.".to_string());
        }

        let no_ms = args[1] == "--no-ms".to_string();
        let destination: Box<path::Path> = Box::from(path::Path::new(&args[if no_ms {2} else {1}]));

        if !destination.exists() {
            return Err("Invalid destination.".to_string());
        }


        Ok(Config { destination, no_ms })
    }
}

fn run() -> Result<(), String> {
    let config: Config = Config::from_args()?;

    println!("{:#?}", config);

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("An error occurred.\n{}", e);
            1
        }
    })
}
