use std::fs;

fn open_file(filepath: &str) -> String {
    let contents = fs::read_to_string(filepath).expect("sigma");
    return contents;
}
fn main() {
    let mut words: Vec<String> = vec![]; // Initializes an empty vector of words
    let mut word: Vec<char> = vec![]; // Initialises the first word
    let contents: String = open_file("./dash/return.dash");

    for c in contents.chars() {
        if c != '\n' && c != ' ' && c != '\t' {
            word.push(c);
        } else if word != vec![] {
            words.push(word.iter().collect());
            word = vec![];
        }
    }

    for x in words {
        print!("{x}\n");
    }
}
