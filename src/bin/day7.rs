
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let lines = File::open("data/day7.txt")
        .iter()
        .flat_map(|file| BufReader::new(file).lines())
        .flat_map(|line| line)
        .map(|line| Line::create(&line))
        .collect::<Vec<Line>>();

    // println!("{:?}", lines);

    let result: u64 = lines
        .iter()
        .filter(|line| validate(&line, 1, line.numbers[0]))
        .inspect(|line| println!("{:?}", line))
        .map(|line| line.test_value)
        .sum();

    println!("result {}", result); //333027885676693
}

fn validate(line: &Line, position: usize, calculated: u64) -> bool {
    if position == line.numbers.len() {
        calculated== line.test_value
    } else if calculated > line.test_value {
        false
    } else {
        let next_val = line.numbers[position];

        let new_calculated = calculated + next_val;
        let mut is_valid = validate(
            line,
            position + 1,
            new_calculated
        );
        if !is_valid {
            let new_calculated = (calculated.to_string() + &next_val.to_string()).parse::<u64>().unwrap();
            is_valid = validate(
                line,
                position + 1,
              new_calculated
            );
        }
        if !is_valid {
            let new_calculated = calculated * next_val;
            is_valid = validate(
                line,
                position + 1,
              new_calculated
            );
        }
        is_valid
    }
}

#[derive(Debug)]
struct Line {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Line {
    fn create(line: &str) -> Self {
        let split_test: Vec<&str> = line.split(":").collect();
        let test_value: u64 = split_test[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = split_test[1]
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        Self {
            numbers,
            test_value,
        }
    }
}
