use regex::Regex;
use std::fs;
fn main() {
    prob2();
}
fn prob2() {
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]+),([0-9]+)\)").expect("Incorrect Regex");
    let data = fs::read_to_string("data/day3.txt").expect("File not found");

    let mut result = 0;
    let mut to_do = true;
    for found in re.captures_iter(&data) {
        println!("found {:?}", found);

        let found_string = found.get(0).unwrap().as_str();

        if found_string.starts_with("mul") && to_do {
            let (val1, val2) = (
                found.get(3).unwrap().as_str(),
                found.get(4).unwrap().as_str(),
            );
            println!(" {val1} {val2}");
            result = result + val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap();
        } else if found_string.starts_with("don't") {
            to_do = false
        } else if found_string.starts_with("do") {
            to_do = true
        }
        println!("{result}")
    }
}

fn prob1() {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Incorrect Regex");
    let data = fs::read_to_string("data/day3.txt").expect("File not found");

    let result: i32 = re
        .captures_iter(&data)
        .inspect(|f| println!("{:?}", f)) //Captures({0: 48..57/"mul(11,8)", 1: 52..54/"11", 2: 55..56/"8"})
        .map(|c| c.extract())
        .map(|(_, [val1, val2])| (val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap()))
        .sum();
    println!("{result}")
}
