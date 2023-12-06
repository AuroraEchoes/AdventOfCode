use std::env;

fn main() {
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "part1" => part_one(),
            "part2" => part_two(),
            _ => panic!("Unknown part!")
        }
    } else {
        println!("Did not supply part. Proceeding with part one.");
        println!("Part 1:");
        part_one();
        println!("Part 2:");
        part_two();
    }
}

fn part_one() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let mut times = numbers(lines.next().unwrap());
    let distances = numbers(lines.next().unwrap());
    let combined = times.iter_mut().enumerate().map(|(i, time)| (*time, distances[i])).collect::<Vec<_>>();
    let mut results = Vec::new();
    for (i, (time, dist)) in combined.iter().enumerate() {
        let mut race_results = Vec::new();
        // Calculate every possible race distance
        for held_secs in 0..=*time {
            let acceleration_ticks = time - held_secs;
            let mut distance = 0;
            // There's an integral way to do this
            // But I am wayyy too tired for that
            for _ in 0..acceleration_ticks {
                distance += held_secs;
            }
            race_results.push((distance, held_secs));
        }
        let beating_results = race_results.iter().filter(|(d, _)| d > dist).count();
        results.push(beating_results);
    }
    println!("Product: {:?}", results.iter().product::<usize>());
}

fn part_two() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let time = number_single(lines.next().unwrap());
    let distance = number_single(lines.next().unwrap());
    let mut race_results = Vec::new();
    // Calculate every possible race distance
     for held_secs in 0..=time {
        let acceleration_ticks = time - held_secs;
        let distance = held_secs * acceleration_ticks;
        race_results.push((distance, held_secs));
    }
    let beating_results = race_results.iter().filter(|(d, _)| *d > distance).count();
    println!("Product: {:?}", beating_results);
}

fn numbers(string: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut buf = 0;
    for c in string.chars() {
        if c.is_digit(10) {
            buf *= 10;
            buf += c.to_digit(10).unwrap();
        } else {
            if buf != 0 {
                numbers.push(buf);
            }
            buf = 0;
        }
    }
    if buf != 0 {
        numbers.push(buf);
    }
    return numbers;
}

fn number_single(string: &str) -> u128 {
    let mut buf = 0;
    for c in string.chars() {
        if c.is_digit(10) {
            buf *= 10;
            buf += c.to_digit(10).unwrap() as u128;
        }
    }
    return buf;
}
