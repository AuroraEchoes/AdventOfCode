use std::{env, num};

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
    let parsed_data = input
        .lines()
        .map(|l| {
            let mut game_split = l.split(':');
            let game_no = number_from_str(game_split.clone().next().unwrap());
            let rounds = game_split.nth(1).unwrap().split(';').collect::<Vec<_>>();
            let mut maxes = [0, 0, 0];
            for r in rounds {
                let cube_split = r.split(',');
                for cubes in cube_split {
                    let (color, number) = parse_cube(cubes);
                    if number > maxes[color] {
                        maxes[color] = number;
                    }
                }
            }
            return (game_no, maxes[0], maxes[1], maxes[2])
        })
        .filter(|x| x.1 <= 12 && x.2 <= 13 && x.3 <= 14)
        .map(|x| x.0)
        .sum::<u32>();
    println!("Sum: {parsed_data:?}");
}

fn part_two() {
    let input = include_str!("../input.txt");
    let parsed_data = input
        .lines()
        .map(|l| {
            let mut game_split = l.split(':');
            let game_no = number_from_str(game_split.clone().next().unwrap());
            let rounds = game_split.nth(1).unwrap().split(';').collect::<Vec<_>>();
            let mut maxes = [0, 0, 0];
            for r in rounds {
                let cube_split = r.split(',');
                for cubes in cube_split {
                    let (color, number) = parse_cube(cubes);
                    if number > maxes[color] {
                        maxes[color] = number;
                    }
                }
            }
            return (game_no, maxes[0], maxes[1], maxes[2])
        })
        .map(|x| x.1 * x.2 * x.3)
        .sum::<u32>();
    println!("Sum: {parsed_data:?}");
}

fn number_from_str(input: &str) -> u32 {
    let digits = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let mut sum = 0u32;
    for (i, d) in digits.iter().enumerate() {
        let exponent = 10_u32.pow(digits.len() as u32 - i as u32 - 1);
        sum += d * exponent;
    }
    return sum; 
}

fn parse_cube(cube: &str) -> (usize, u32) {
     let mut color = 10;
     if cube.contains("red") { color = 0 }
     if cube.contains("green") { color = 1 }
     if cube.contains("blue") { color = 2 }
     return (color, number_from_str(cube));
}
