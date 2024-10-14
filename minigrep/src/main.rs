use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
