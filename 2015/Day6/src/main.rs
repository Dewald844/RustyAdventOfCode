use std::fs::File;
use std::io::Read;
use std::path::Path;

fn create_grid() -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for _ in 0..1000 {
        let mut row = Vec::new();
        for _ in 0..1000 {
            row.push(false);
        }
        grid.push(row);
    }
    grid
}

fn create_brightness_grid() -> Vec<Vec<usize>> {
    let mut grid = Vec::new();
    for _ in 0..1000 {
        let mut row = Vec::new();
        for _ in 0..1000 {
            row.push(0);
        }
        grid.push(row);
    }
    grid
}

fn turn_on(grid: &mut Vec<Vec<bool>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            grid[i][j] = true;
        }
    }
}

fn increment_brightness(grid: &mut Vec<Vec<usize>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            grid[i][j] += 1;
        }
    }
}

fn turn_off(grid: &mut Vec<Vec<bool>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            grid[i][j] = false;
        }
    }
}

fn decrement_brightness(grid: &mut Vec<Vec<usize>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            if grid[i][j] > 0 {
                grid[i][j] -= 1;
            }
        }
    }
}

fn toggle(grid: &mut Vec<Vec<bool>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            grid[i][j] = !grid[i][j];
        }
    }
}

fn increment_brightness_by_two(grid: &mut Vec<Vec<usize>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for i in x1..=x2 {
        for j in y1..=y2 {
            grid[i][j] += 2;
        }
    }
}

fn count_lights_on(grid: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] {
                count += 1;
            }
        }
    }
    count
}

fn count_brightness(grid: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            count += grid[i][j];
        }
    }
    count
}

fn read_coords(input: &str) -> (usize, usize, usize, usize) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let coords: Vec<&str> = words[2].split(',').collect();
    let x1: usize = coords[0].parse().unwrap();
    let y1: usize = coords[1].parse().unwrap();
    let coords: Vec<&str> = words[4].split(',').collect();
    let x2: usize = coords[0].parse().unwrap();
    let y2: usize = coords[1].parse().unwrap();
    (x1, y1, x2, y2)
}

fn read_toggle_coords(input: &str) -> (usize, usize, usize, usize) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let coords: Vec<&str> = words[1].split(',').collect();
    let x1: usize = coords[0].parse().unwrap();
    let y1: usize = coords[1].parse().unwrap();
    let coords: Vec<&str> = words[3].split(',').collect();
    let x2: usize = coords[0].parse().unwrap();
    let y2: usize = coords[1].parse().unwrap();
    (x1, y1, x2, y2)
}

fn part_one(input: &str) -> usize {
    let mut grid = create_grid();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "turn" => {
                let (x1, y1, x2, y2) = read_coords(line);
                match words[1] {
                    "on" => turn_on(&mut grid, x1, y1, x2, y2),
                    "off" => turn_off(&mut grid, x1, y1, x2, y2),
                    _ => (),
                }
            }
            "toggle" => {
                let (x1, y1, x2, y2) = read_toggle_coords(line);
                toggle(&mut grid, x1, y1, x2, y2);
            }
            _ => (),
        }
    }
    count_lights_on(&grid)
}

fn part_two(input: &str) -> usize {
    let mut grid = create_brightness_grid();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "turn" => {
                let (x1, y1, x2, y2) = read_coords(line);
                match words[1] {
                    "on" => increment_brightness(&mut grid, x1, y1, x2, y2),
                    "off" => decrement_brightness(&mut grid, x1, y1, x2, y2),
                    _ => (),
                }
            }
            "toggle" => {
                let (x1, y1, x2, y2) = read_toggle_coords(line);
                increment_brightness_by_two(&mut grid, x1, y1, x2, y2);
            }
            _ => (),
        }
    }
    count_brightness(&grid)
}

fn main() {
    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));
}
