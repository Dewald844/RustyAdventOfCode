use md5;

fn check_first_number_of_chars (number : i32, hash: &str) -> bool {
    let mut chars = hash.chars();
    let mut i = 0;
    while i < number {
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

fn compute_md5_hash (input: &str, count: u32) -> String {
    let to_compute = format!("{}{}", input, count);
    let digest = md5::compute(to_compute);
    format!("{:x}", digest)
}

fn part_one (input: &str) -> u32 {
    let mut hash = String::new();
    let mut i = 0;

    while !check_first_number_of_chars(5, &hash) {
        i += 1;
        hash = compute_md5_hash(input, i);
    }

    i
}

fn part_two (input: &str) -> u32 {
    let mut hash = String::new();
    let mut i = 0;

    while !check_first_number_of_chars(6, &hash) {
        i += 1;
        hash = compute_md5_hash(input, i);
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