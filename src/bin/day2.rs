use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
fn main() {
    let file = File::open("data/day2.txt").expect("File not found");
    let reader = BufReader::new(file);
    let result1: i32 = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|f| f.parse::<i32>().expect("Couldnt parse numbers"))
                .collect::<Vec<i32>>()
        })
        .map(|v| is_safe2(&v))
        .sum();

    println!("result1 {}", result1)
}
fn is_safe2(vec: &Vec<i32>) -> i32 {
    let result = is_safe(vec);

    if result == 1 {
        result
    } else {
        let len = vec.len();

        let safe = (0..len)
            .map(|i| remove(vec, i))
            .map(|v| is_safe(&v))
            .any(|r| r == 1);
        if safe {
            1
        } else {
            0
        }
    }
}
fn remove(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter_map(|(i, val)| if i != index { Some(*val) } else { None })
        .collect()
}

fn is_safe(vec: &Vec<i32>) -> i32 {
    println!("[{:?}] ", vec);
    let safe = vec.windows(3).all(|s| compare3(s));

    if safe {
        1
    } else {
        0
    }
}

fn compare3(a: &[i32]) -> bool {
    let (first, second, third) = (a[0], a[1], a[2]);

    let diff1 = first - second;
    let diff2 = second - third;

    let safe = diff1 != 0
        && diff1.abs() <= 3
        && diff2 != 0
        && diff2.abs() <= 3
        && ((diff1 > 0 && diff2 > 0) || (diff1 < 0 && diff2 < 0));
    println!("[{:?}] {} {} {}", a, diff1, diff2, safe);
    safe
}
