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
    let input = include_str!("../input.txt");
    let sum: i32 = input
        .lines()
        .map(|l| l.chars()
             .filter_map(|c| char_to_int(&c))
             .collect::<Vec<_>>())
        .map(|vec| {
            let count = 10 * vec.first().unwrap() + vec.last().unwrap();
            count
        })
        .sum();
    println!("Sum: {:?}", sum);
}

fn char_to_int(c: &char) -> Option<i32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

// To tidy... like hell
fn word_to_char(acc: &str) -> Option<char> {
    if ends_with(acc, "one") {
        return Some('1');
    }
    if ends_with(acc, "two") {
        return Some('2')
    }
    if ends_with(acc, "three") {
        return Some('3')
    }
    if ends_with(acc, "four") {
        return Some('4')
    }
    if ends_with(acc, "five") {
        return Some('5')
    }
    if ends_with(acc, "six") {
        return Some('6')
    }
    if ends_with(acc, "seven") {
        return Some('7')
    }
    if ends_with(acc, "eight") {
        return Some('8')
    }
    if ends_with(acc, "nine") {
        return Some('9')
    }
    return None;
}

fn ends_with(string: &str, contains: &str) -> bool {
    println!("{string}, {contains}");
    if string.len() < contains.len() {
        return false;
    }
    let cont_len = string.len() - contains.len();
    let mut cont_string = String::new();
    string
        .char_indices()
        .filter(|(i, _)| i >= &cont_len)
        .for_each(|(_, c)| cont_string += c.to_string().as_str());

    println!("{string}, {cont_string}");
    return string == cont_string.as_str();
}

fn part_two() {
    let input = include_str!("../input.txt");
    let mut acc = String::new();
    let sum: i32 = input
        .lines()
        .map(|l| l.chars()
             .filter_map(|c| {
                acc = format!("{acc}{c}");
                match word_to_char(acc.as_str()) {
                    Some(c) => { acc = String::new(); char_to_int(&c) }
                    None => char_to_int(&c)
                }
            })
            .collect::<Vec<_>>())
        .map(|vec| {
            let count = 10 * vec.first().unwrap() + vec.last().unwrap();
            count
        })
        .sum();
    println!("Sum: {:?}", sum);
}
