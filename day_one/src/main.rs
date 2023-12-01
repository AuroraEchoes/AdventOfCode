use std::env;

fn main() {
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "part1" => part_one(),
            "part2" => part_two(),
            _ => panic!("Part unknown") 
        }
    } else {
        println!("Did not supply part. Proceeding with part one.");
        part_one();
    }
}

fn part_one() {
    println!("Part one!");
}

fn part_two() {
    println!("Part two!");
}
