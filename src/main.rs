use std::fs;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = &args[2];
    let query = &args[1];
    let words = fs::read_to_string(fname).expect("");
    for word in words.lines() {
        if word.contains(query) {
            println!("{}", word);
        }
    }
}
