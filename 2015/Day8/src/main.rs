use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_code_chars = 0;
    let mut total_encoded_chars = 0;

    for line in reader.lines() {
        let line = line?;
        total_code_chars += line.len();
        total_encoded_chars += get_encoded_length(&line);
    }

    let difference = total_encoded_chars - total_code_chars;
    println!("Difference: {}", difference);

    Ok(())
}

fn get_encoded_length(input: &str) -> usize {
    let mut encoded = String::new();
    encoded.push('"');
    for c in input.chars() {
        match c {
            '\\' => encoded.push_str("\\\\"),
            '"' => encoded.push_str("\\\""),
            _ => encoded.push(c),
        }
    }
    encoded.push('"');
    encoded.len()
}