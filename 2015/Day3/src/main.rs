use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinates {
    x_pos: i32,
    y_pos: i32,
}

fn move_to_next_location (current_location: Coordinates, direction: char) -> Coordinates {
    let mut new_location: Coordinates = current_location;

    match direction {
        '^' => new_location.y_pos += 1,
        'v' => new_location.y_pos -= 1,
        '>' => new_location.x_pos += 1,
        '<' => new_location.x_pos -= 1,
        _ => println!("Invalid direction")
    }

    new_location
}

fn count_distinct_elements (list: Vec<Coordinates>) -> i32 {
    let distinct_elements: HashSet<Coordinates> = list.into_iter().collect();
    distinct_elements.len() as i32
}

fn part_one (directions: Vec<char>) -> i32 {

    let mut santa_location: Coordinates = Coordinates { x_pos: 0, y_pos: 0 };
    let mut visited_locations: Vec<Coordinates> = vec![santa_location];

    for &direction in &directions {
        santa_location = move_to_next_location(santa_location, direction);
        visited_locations.push(santa_location);
    }

    count_distinct_elements(visited_locations)
}

fn part_two (directions: Vec<char>) -> i32 {

    let mut santa_location: Coordinates = Coordinates { x_pos: 0, y_pos: 0 };
    let mut robot_location: Coordinates = Coordinates { x_pos: 0, y_pos: 0 };
    let mut visited_locations: Vec<Coordinates> = vec![santa_location];

    for (index, &direction) in directions.iter().enumerate() {
        if index % 2 == 0 {
            santa_location = move_to_next_location(santa_location, direction);
            visited_locations.push(santa_location);
        } else {
            robot_location = move_to_next_location(robot_location, direction);
            visited_locations.push(robot_location);
        }
    }

    count_distinct_elements(visited_locations)
}

fn main() {
    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    // Casting file String to Vector(Array)
    let directions: Vec<char> = contents.chars().collect();

    let part_one_answer: i32 = part_one(directions.clone());
    let part_two_answer: i32 = part_two(directions.clone());

    println!("Part one : {:?}", part_one_answer);
    println!("Part two : {:?}", part_two_answer);
}