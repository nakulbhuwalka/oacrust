use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("data/day4.txt").expect("File not found");

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect();
    let row_count = lines.len();
    let col_count = lines[0].len();

    let mut result = 0;

    for row in 0..row_count {
        for col in 0..(col_count - 3) {
            let line = &lines[row];
            let slice = line.get(col..(col + 4)).unwrap();
            if match_xmas(slice) {
                result += 1;
                //println!("{row} {col}")
            }
        }
    }
    println!("Done 1");
    for col in 0..col_count {
        for row in 0..row_count - 3 {
            let mut string = String::new();
            string.push(lines[row].chars().nth(col).unwrap());
            string.push(lines[row + 1].chars().nth(col).unwrap());
            string.push(lines[row + 2].chars().nth(col).unwrap());
            string.push(lines[row + 3].chars().nth(col).unwrap());

            if match_xmas(string.as_str()) {
                result += 1;
                //println!("{row} {col}")
            }
        }
    }
    println!("Done 2");
    for row in 0..row_count - 3 {
        for col in 0..(col_count - 3) {
            let mut string = String::new();
            string.push(lines[row].chars().nth(col).unwrap());
            string.push(lines[row + 1].chars().nth(col + 1).unwrap());
            string.push(lines[row + 2].chars().nth(col + 2).unwrap());
            string.push(lines[row + 3].chars().nth(col + 3).unwrap());

            if match_xmas(string.as_str()) {
                result += 1;
                //println!("{row} {col}")
            }
        }
    }
    println!("Done 3");
    for col in 3..(col_count) {
        for row in 0..row_count - 3 {
            let mut string = String::new();
            string.push(lines[row].chars().nth(col).unwrap());
            string.push(lines[row + 1].chars().nth(col - 1).unwrap());
            string.push(lines[row + 2].chars().nth(col - 2).unwrap());
            string.push(lines[row + 3].chars().nth(col - 3).unwrap());

            if match_xmas(string.as_str()) {
                result += 1;
                // println!("{row} {col}")
            }
        }
    }
    println!("Result {result}");
    let mut result2 = 0;
    for col in 1..(col_count - 1) {
        for row in 1..row_count - 1 {
            let mut string1 = String::new();
            let mut string2 = String::new();
            string1.push(lines[row - 1].chars().nth(col - 1).unwrap());
            string1.push(lines[row].chars().nth(col).unwrap());
            string1.push(lines[row + 1].chars().nth(col + 1).unwrap());

            string2.push(lines[row - 1].chars().nth(col + 1).unwrap());
            string2.push(lines[row].chars().nth(col).unwrap());
            string2.push(lines[row + 1].chars().nth(col - 1).unwrap());

            if match_mas(string1.as_str()) && match_mas(string2.as_str()) {
                result2 += 1;
                // println!("{row} {col}")
            }
        }

    }
    println!("Result {result2}");
}

fn match_xmas(slice: &str) -> bool {
    if slice == "XMAS" || slice == "SAMX" {
        true
    } else {
        false
    }
}

fn match_mas(slice: &str) -> bool {
    if slice == "MAS" || slice == "SAM" {
        true
    } else {
        false
    }
}
