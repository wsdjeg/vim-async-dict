use std::fs;
#[allow(unused_imports)]
use std::process;
fn main() {
    #[allow(unused_variables)]
    let args: Vec<String> = std::env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);
        // process::exit(1);
    // });
    let config = Config {
        query: String::from("ru"),
        filename: String::from(r"C:\Users\wsdjeg\DotFiles\dict\words.txt"),
    };
    let words = fs::read_to_string(config.filename).expect("");
    find_start(&words, &config.query);
    for word in words.lines() {
        if word.contains(&config.query) && !word.starts_with(&config.query) {
            println!("{}", word);
        }
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    #[allow(dead_code)]
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn find_start(words: &String, query: &String) {
    #[allow(unused_variables)]
    let i: i32 = query.len() as i32;
    for word in words.lines() {
        if word.starts_with(&query[..1]) && word.contains(&query.clone()) {
            println!("{}", word);
        }
    }
}
