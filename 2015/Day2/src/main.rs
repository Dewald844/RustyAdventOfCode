use std::fs::File;
use std::io::Read;
use std::path::Path;

fn compute_smallest_side_area (l: i32,w: i32,h: i32) -> i32 {
    let mut vec: Vec<i32> = vec![l,w,h];
    vec.sort();
    let slack = vec[0] * vec[1];
    slack
}

fn compute_wrapping_total (l: i32,w: i32,h: i32) -> i32 {
    let wrapping_area: i32 = (2*l*w) + (2*w*h) + (2*h*l);
    let slack: i32 = compute_smallest_side_area(l,w,h);
    let total_needed_paper = wrapping_area + slack;
    total_needed_paper
}

fn compute_ribbon_total (l: i32,w: i32,h: i32) -> i32 {
    let mut vec: Vec<i32> = vec![l,w,h];
    vec.sort();
    let ribbon_to_wrap = (vec[0] * 2) + (vec[1] * 2);
    let ribbon_for_bow = l * w * h;
    let total_ribbon = ribbon_to_wrap + ribbon_for_bow;
    total_ribbon
}

fn compute_part_one (gifts_to_wrap: Vec<&str>) -> i32 {

    let mut total_wrapping = 0;

    for gift in gifts_to_wrap {
        let dimentions: Vec<&str> = gift.split("x").collect();
        let length = dimentions[0].parse::<i32>().unwrap();
        let width  = dimentions[1].parse::<i32>().unwrap();
        let height = dimentions[2].parse::<i32>().unwrap();

        let gift_total_wrap = compute_wrapping_total(length, width, height);

        total_wrapping = total_wrapping + gift_total_wrap;
    }

    total_wrapping
}

fn compute_part_two (gifts_to_wrap: Vec<&str>) -> i32 {
    let mut total_ribbon = 0;

    for gift in gifts_to_wrap {
        let dimentions: Vec<&str> = gift.split("x").collect();
        let length = dimentions[0].parse::<i32>().unwrap();
        let width  = dimentions[1].parse::<i32>().unwrap();
        let height = dimentions[2].parse::<i32>().unwrap();

        let gift_total_ribbon = compute_ribbon_total(length, width, height);

        total_ribbon = total_ribbon + gift_total_ribbon;
    }

    total_ribbon

}

fn main() {

    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let gifts_to_wrap: Vec<&str> = contents.split('\n').collect();

    let test : Vec<&str> = vec!["1x1x10"];

    let part_one_answer = compute_part_one(gifts_to_wrap.clone());
    let part_two_answer = compute_part_two(gifts_to_wrap.clone());

    println!("Part 1 : {}", part_one_answer);
    println!("Part 2 : {}", part_two_answer);

}
