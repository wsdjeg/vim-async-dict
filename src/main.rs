use std::fs;
#[allow(unused_imports)]
use std::process;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == String::from("-v") {
            version();
            process::exit(0);
        } else if args[1] == String::from("-h") {
            help();
            process::exit(0);
        }
    }
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let config = Config {
    // query: String::from("ru"),
    // filename: String::from(r"C:\Users\wsdjeg\DotFiles\dict\words.txt"),
    // };
    let words = fs::read_to_string(config.filename).expect("");
    let another: Vec<Vec<String>> = find_start(&words, &config.query);
    for words in another {
        for word in words {
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

// find the words start with query.
fn find_start(words: &String, query: &String) -> Vec<Vec<String>> {
    let mut rst: Vec<Vec<String>> = vec![];
    // 首字母匹配 + contains
    let mut words1: Vec<String> = vec![];
    //  contains
    let mut another: Vec<String> = vec![];
    for word in words.lines() {
        if word.starts_with(&query[..]) {
            println!("{}", word);
        }else if word.starts_with(&query[..1]) && word.contains(&query[1..]) {
            words1.push(String::from(word));
        }else if word.contains(&query[..]){
            another.push(String::from(word));
        }
    }
    rst.push(words1);
    rst.push(another);
    rst
}

fn version() {
    let version = "v0.1.0";
    println!("minigrep: {}", version);
}

fn help() {
    version();
    println!("{}", "USAGE");
    println!("{}", "      minigrep <query> <filename>");
}

#[allow(dead_code)]
fn split_word(st: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    for c in st.chars() {
        if c >= 'a' && c <= 'z' {
            word.push(c);
        } else if c >= 'A' && c <= 'Z' {
            if !word.is_empty() {
                words.push(word.clone());
                word.clear();
            }
            word.push(c);
        }
    }
    if !word.is_empty() {
        words.push(word.clone());
    }
    words
}

#[test]
fn split_word_test() {
    let str = "HelloWorld";
    let mut rst: Vec<String> = Vec::new();
    rst.push("Hello".to_string());
    rst.push("World".to_string());
    assert_eq!(rst, split_word(str));
}
