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
    let sum: u32 = input
        .lines()
        .map(|l| l.chars()
             .filter_map(|c| c.to_digit(10))
             .collect::<Vec<_>>())
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();
    println!("Sum: {:?}", sum);
}

fn part_two() {
    let input = include_str!("../input.txt");
    let n: u32 = input
        .lines()
        .map(search_line)
        .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
        .sum();
    println!("Sum: {n:?}")
}

fn word_to_int(acc: &str) -> Option<u32> {
    if acc.contains("one") {
        return Some(1);
    }
    if acc.contains("two") {
        return Some(2);
    }
    if acc.contains("three") {
        return Some(3);
    }
    if acc.contains("four") {
        return Some(4);
    }
    if acc.contains("five") {
        return Some(5);
    }
    if acc.contains("six") {
        return Some(6);
    }
    if acc.contains("seven") {
        return Some(7);
    }
    if acc.contains("eight") {
        return Some(8);
    }
    if acc.contains("nine") {
        return Some(9);
    }
    return None;
}

fn search_line(line: &str) -> Vec<u32> {
    let mut buffer = String::new();
    let mut digits = Vec::new();
    for c in line.chars() {
        buffer += c.to_string().as_str();
        let char_digit = c.to_digit(10);
        let buf_digit = word_to_int(buffer.as_str());
        if let Some(d) = char_digit {
            digits.push(d);
            buffer = String::new();
        }
        if let Some(bc) = buf_digit {
            digits.push(bc);
            let residual_char = buffer.chars().last().unwrap();
            buffer = residual_char.to_string();
        }
    }
    return digits;
}

