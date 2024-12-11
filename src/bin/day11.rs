use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
fn main() {
    prob1();
}
fn prob1() {
    let file = File::open("data/day11.txt").expect("File not found");

    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    reader.read_to_string(&mut buf).expect("should work");

    let mut stones = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cache: HashMap<(u8, u64), u64> = HashMap::new();
    println!(
        "result prob2{}",
        stones
            .iter()
            .map(|stone| blink_recursive_cached(stone, 75, &mut cache))
            .sum::<u64>()
    );

    
    for i in 0..25 {
        stones = blink(&stones);
    }

    println!("stones prob1 {:?}", stones.len()); //stones 186424
}

fn blink_recursive_cached(stone: &u64, remaining: u8, cache: &mut HashMap<(u8, u64), u64>) -> u64 {
    let result = if remaining == 0 {
        1
    } else if let Some(len) = cache.get(&(remaining, *stone)) {
        *len
    } else if *stone == 0 {
        blink_recursive_cached(&1, remaining - 1, cache)
    } else if let Some(len) = is_even(stone) {
        let new_stones = split(stone, &len);
        let new_remainin = remaining - 1;
        blink_recursive_cached(&new_stones.0, new_remainin, cache)
            + blink_recursive_cached(&new_stones.1, new_remainin, cache)
    } else {
        blink_recursive_cached(&(stone * 2024), remaining - 1, cache)
    };
    cache.insert((remaining, *stone), result);
    result
}



fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();

    for old_stone in stones {
        if *old_stone == 0 {
            new_stones.push(1);
        } else if let Some(len) = is_even(old_stone) {
            let t_add = split(old_stone, &len);
            new_stones.push(t_add.0);
            new_stones.push(t_add.1);
        } else {
            new_stones.push(old_stone * 2024);
        }
    }

    new_stones
}

fn split(num: &u64, len: &u32) -> (u64, u64) {
    let split_num = 10u64.pow(len / 2);
    (num / split_num, num % split_num)
}

fn is_even(num: &u64) -> Option<u32> {
    let len = num.ilog10() + 1;
    if len % 2 == 0 {
        Some(len)
    } else {
        None
    }
}
