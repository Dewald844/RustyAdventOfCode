use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn parse_instruction(instruction : &str) -> (String, String) {
    let parts: Vec<&str> = instruction.split(" -> ").collect();
    (parts[1].to_string(), parts[0].to_string())
}

fn evaluate(wire: &str, instructions: &HashMap<String, String>, cache: &mut HashMap<String, u16>) -> u16 {
    if let Ok(value) = wire.parse::<u16>() {
        return value;
    }

    if let Some(&cached_value) = cache.get(wire) {
        return cached_value;
    }

    let instruction = instructions.get(wire).unwrap();
    let value = if instruction.contains("AND") {
        let parts: Vec<&str> = instruction.split(" AND ").collect();
        evaluate(parts[0], instructions, cache) & evaluate(parts[1], instructions, cache)
    } else if instruction.contains("OR") {
        let parts: Vec<&str> = instruction.split(" OR ").collect();
        evaluate(parts[0], instructions, cache) | evaluate(parts[1], instructions, cache)
    } else if instruction.contains("LSHIFT") {
        let parts: Vec<&str> = instruction.split(" LSHIFT ").collect();
        evaluate(parts[0], instructions, cache) << parts[1].parse::<u32>().unwrap()
    } else if instruction.contains("RSHIFT") {
        let parts: Vec<&str> = instruction.split(" RSHIFT ").collect();
        evaluate(parts[0], instructions, cache) >> parts[1].parse::<u32>().unwrap()
    } else if instruction.contains("NOT") {
        let parts: Vec<&str> = instruction.split("NOT ").collect();
        !evaluate(parts[1], instructions, cache)
    } else {
        evaluate(instruction, instructions, cache)
    };

    cache.insert(wire.to_string(), value);
    value
}

fn test_input() -> Vec<&'static str> {
    vec![
        "123 -> x",
        "456 -> y",
        "x AND y -> d",
        "x OR y -> e",
        "x LSHIFT 2 -> f",
        "y RSHIFT 2 -> g",
        "NOT x -> h",
        "NOT y -> i"
    ]
}



fn main() {
    // Reading file contents
    let path = Path::new("input.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let mut instructions = HashMap::new();
    for line in contents.split("\n") {
        let (wire, instruction) = parse_instruction(line);
        instructions.insert(wire, instruction);
    }

    let mut cache = HashMap::new();
    let _ = &cache.insert("b".to_string(), 16076);
    let signal_a = evaluate("a", &instructions, &mut cache);
    println!("Signal to wire a: {}", signal_a);
}
