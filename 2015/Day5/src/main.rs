use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

fn check_three_vowels (input: &str) -> bool {
    let re = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    re.is_match(input)
}

fn check_double_letter (input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn check_no_bad_strings (input: &str) -> bool {
    let re = Regex::new(r"ab|cd|pq|xy").unwrap();
    !re.is_match(input)
}

fn part_one (input: &str) -> u32 {
    let mut nice_strings = 0;
    for line in input.lines() {
        if check_three_vowels(line) && check_double_letter(line) && check_no_bad_strings(line) {
            nice_strings += 1;
        }
    }
    nice_strings
}

fn check_two_pairs(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let mut pairs = HashMap::new();

    for i in 0..chars.len() - 1 {
        let pair = (chars[i], chars[i + 1]);

        match pairs.get(&pair) {
            Some(&index) if i - index > 1 => return true,
            _ => pairs.insert(pair, i),
        };
    }

    false
}

fn check_repeats_with_one_between(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn part_two (input: &str) -> u32 {
    let mut nice_strings = 0;
    for line in input.lines() {
        if check_two_pairs(line) && check_repeats_with_one_between(line) {
            nice_strings += 1;
        }
    }
    nice_strings
}

fn main (){

    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));

}