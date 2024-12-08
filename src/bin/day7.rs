use std::fmt::format;
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
        .filter(|line| validate(&line, 0, None, &None))
        .inspect(|line| println!("{:?}", line))
        .map(|line| line.test_value)
        .sum();

    println!("result {}", result);
}

fn validate(line: &Line, position: usize, calculated: Option<u64>, log: &Option<String>) -> bool {
    if position == line.numbers.len() {
        calculated.is_some_and(|c| c == line.test_value)
    } else if calculated.is_some_and(|c| c > line.test_value) {
        false
    } else {
        let next_val = line.numbers[position];
        let new_log = match log {
            None => "",
            Some(s) => s,
        };

        let new_calculated = calculated.unwrap_or(0) + next_val;
        let mut is_valid = validate(
            line,
            position + 1,
            Some(new_calculated),
            &Some(format!("{} + {}", new_log, next_val)),
        );
        if !is_valid {
            let new_calculated = calculated
                .map(|last_val| last_val.to_string() + &next_val.to_string())
                .map_or(next_val, |s| s.parse::<u64>().unwrap());
            is_valid = validate(
                line,
                position + 1,
                Some(new_calculated),
                &Some(format!("{}{}", new_log, next_val)),
            );
        }
        if !is_valid {
            let new_calculated = calculated.unwrap_or(1) * next_val;
            is_valid = validate(
                line,
                position + 1,
                Some(new_calculated),
                &Some(format!("{} * {}", new_log, next_val)),
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
