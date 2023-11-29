
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[warn(unused_must_use)]

fn part_one (instructions: Vec<char>) -> i32 {

    let mut floor = 0;

    for &c in &instructions {
        if c == ')' {
            floor = floor - 1;
        }
        else {
            floor = floor + 1;
        }
    }

    floor
}

fn part_two (instructions: Vec<char>) -> i32 {

    let mut floor = 0;
    let mut index = 1;

    for &c in &instructions {
        if c == ')' {
            floor = floor - 1;
        }
        else {
            floor = floor + 1;
        }

        if &floor < &0 {
            break;
        }

        index = index + 1;
    }

    index

}

fn main() {
    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    // Casting file String to Vector(Array)
    let instructions: Vec<char> = contents.chars().collect();

    let part_one_answer: i32 = part_one(instructions.clone());
    let part_two_answer: i32 = part_two(instructions.clone());

    println!("Part one : {:?}", part_one_answer);
    println!("Part two : {:?}", part_two_answer);
}
