use md5;

fn check_first_five (hash: &str) -> bool {
    let mut chars = hash.chars();
    let mut i = 0;
    while i < 5 {
        match chars.next() {
            Some(c) => {
                if c != '0' {
                    return false;
                }
            },
            None => return false,
        }
        i += 1;
    }
    true
}

fn check_first_six (hash: &str) -> bool {
    let mut chars = hash.chars();
    let mut i = 0;
    while i < 6 {
        match chars.next() {
            Some(c) => {
                if c != '0' {
                    return false;
                }
            },
            None => return false,
        }
        i += 1;
    }
    true
}

fn part_one (input: &str) -> u32 {
    let mut hash = String::new();
    let mut i = 0;

    while !check_first_five(&hash) {
        i += 1;
        let input_string = format!("{}{}", input, i);
        let digest = md5::compute(input_string);
        hash = format!("{:x}", digest);
    }

    i
}

fn part_two (input: &str) -> u32 {
    let mut hash = String::new();
    let mut i = 0;

    while !check_first_six(&hash) {
        i += 1;
        let input_string = format!("{}{}", input, i);
        let digest = md5::compute(input_string);
        hash = format!("{:x}", digest);
    }

    i
}

fn main() {
    let input = "iwrupvqb";
    let answer1 = part_one(input);
    println!("Part One: {}", answer1);
    let answer2 = part_two(input);
    println!("Part Two: {}", answer2);
}