use std::fs::File;
use std::i64;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    prob1();
}
fn prob1() {
    let button_a = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let button_b = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let file = File::open("data/day13.txt").expect("File not found");

    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect::<Vec<String>>();

    let mut costs = Vec::new();
    for row in (0..lines.len()).step_by(4) {
        let captures = button_a.captures(&lines[row]).unwrap();
        let ax = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let ay = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let captures = button_b.captures(&lines[row + 1]).unwrap();
        let bx = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let by = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let captures = prize.captures(&lines[row + 2]).unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;
        let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;

        let cost = find_min_cost2(ax, bx, x, ay, by, y);
        cost.into_iter().for_each(|cost| costs.push(cost));
        println!("{ax} {ay} {bx} {by} {x} {y} {:?}", cost);
    }
    costs.sort();
    println!("{:?}", costs.iter().sum::<i64>());
}
fn find_min_cost2(ax: i64, bx: i64, x: i64, ay: i64, by: i64, y: i64) -> Option<i64> {
    let moves = find_moves(ax, bx, x, ay, by, y);
    moves
        //.filter(|(a, b)| *a <= 100 && *b <= 100)
        .map(|(a, b)| a * 3 + b)
}

// fn find_min_cost(ax: i64, bx: i64, x: i64, ay: i64, by: i64, y: i64) -> Option<(i64)> {
//     (0..100)
//         .filter_map(|a| find_b(ax, bx, x, ay, by, y, a).map(|b| a * 3 + b))
//         .min()
// }

// fn find_b(ax: i64, bx: i64, x: i64, ay: i64, by: i64, y: i64, a: i64) -> Option<i64> {
//     let b_option = if bx != 0 {
//         Some((x - a * ax) / bx)
//     } else if by != 0 {
//         Some((y - a * ay) / by)
//     } else {
//         None
//     };

//     b_option
//         .filter(|b| a * ax + b * bx == x)
//         .filter(|b| a * ay + b * by == y)
// }

fn find_moves(ax: i64, bx: i64, x: i64, ay: i64, by: i64, y: i64) -> Option<(i64, i64)> {
    let a_num = x * by - y * bx;
    let a_denom = ax * by - ay * bx;

    let b_num = x * ay - y * ax;
    let b_denom = bx * ay - by * ax;

    if b_denom == 0 || a_denom == 0 {
        None
    } else if a_num % a_denom == 0 && b_num % b_denom == 0 {
        Some((a_num / a_denom, b_num / b_denom))
    } else {
        None
    }
}

// fn find_moves(machine: &Machine) -> Option<(i64, i64)> {
//     let a = machine.x * machine.by - machine.y * machine.bx;
//     let b = machine.y * machine.ax - machine.x * machine.ay;
//     let gcd = calc_gcd(a, b);
//     println!("{a} {b} {gcd} {} {}", a / gcd, b / gcd);

//     None
// }

// fn calc_gcd(a: i64, b: i64) -> i64 {
//     if a == b {
//         a
//     } else if a > b {
//         calc_gcd(a - b, b)
//     } else {
//         calc_gcd(a, b - a)
//     }
// }
