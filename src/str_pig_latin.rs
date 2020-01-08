use std::io;

pub fn main() {
    println!("str_pig_latin\n=============");
    println!("Enter a word to translate in pig latin:");

    loop {
        let mut word = String::new();
        io::stdin().read_line(&mut word)
            .expect("Failed to read the line");

        // TODO: check only one word (use split())
        let word = word.trim();
        if word.split_ascii_whitespace().collect::<Vec<&str>>().len() == 1 {
            println!("{} becomes {}", word, pigish(&word[..]));
            break;
        } else {
            println!("Input should be one word.");
        }
    }
}

fn pigish(word: &str) -> String {
    let first_char = first_char(word);
    let pig_latin = match first_char {
        'a'|'e'|'i'|'o'|'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", remaining(word), first_char)
    };
    pig_latin
}

fn first_char(word: &str) -> char {
    let c = word.chars().next().unwrap();
    c
}

fn remaining(word: &str) -> &str {
    let (_, remain) = word.split_at(1);
    remain
}
