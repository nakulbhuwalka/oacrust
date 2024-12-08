use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = File::open("data/day8.txt")
        .iter()
        .flat_map(|file| BufReader::new(file).lines())
        .flat_map(|x| x)
        .collect::<Vec<String>>();

    let max_y = lines.len() as i32;
    let max_x = lines[0].len() as i32;

    let antennas = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphanumeric())
                .map(|(x, freq)| (x as i32, y as i32, freq.clone()))
                .map(|(x, y, freq)| Antenna { x, y, freq })
                .collect::<Vec<Antenna>>()
        })
        .collect::<Vec<Antenna>>();

    let mut result = HashSet::new();
    for antenna1 in antennas.iter() {
        for antenna2 in antennas.iter() {
            get_nodes(antenna1, antenna2, max_x, max_y)
                .iter()
                //.inspect(|x| println!("{:?}", x))
                .for_each(|x| {
                    result.insert(*x);
                });
        }
    }

    println!("{:?}", result.len());

    let mut result = HashSet::new();
    for antenna1 in antennas.iter() {
        for antenna2 in antennas.iter() {
            get_resonant_nodes(antenna1, antenna2, max_x, max_y)
                .iter()
                //.inspect(|x| println!("{:?}", x))
                .for_each(|x| {
                    result.insert(*x);
                });
        }
    }

    println!("{:?}", result.len());
}

fn get_nodes(antenna1: &Antenna, antenna2: &Antenna, max_x: i32, max_y: i32) -> Option<(i32, i32)> {
    if antenna1.freq == antenna2.freq && antenna1 != antenna2 {
        let x = 2 * antenna2.x - antenna1.x;
        let y = 2 * antenna2.y - antenna1.y;

        if x >= 0 && y >= 0 && x < max_x && y < max_y {
            //println!(" Matched {:?} {:?} ({},{})", antenna1, antenna2, x, y);
            Some((x, y))
        } else {
            None
        }
    } else {
        None
    }
}

fn get_resonant_nodes(
    antenna1: &Antenna,
    antenna2: &Antenna,
    max_x: i32,
    max_y: i32,
) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    if antenna1.freq == antenna2.freq && antenna1 != antenna2 {
        let mut distance = 0;
        loop {
            let x = antenna2.x + distance * (antenna2.x - antenna1.x);
            let y = antenna2.y + distance * (antenna2.y - antenna1.y);
            distance = distance + 1;
            if x >= 0 && y >= 0 && x < max_x && y < max_y {
                println!(" Matched {:?} {:?} ({},{})", antenna1, antenna2, x, y);
                result.push((x, y));
            } else {
                break;
            }
        }
    }
    result
}

#[derive(Debug, PartialEq)]
struct Antenna {
    freq: char,
    x: i32,
    y: i32,
}
