use std::collections::HashSet;

use regex::Regex;
use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let robots = &get_data();
    println!("{:?}", robots);

    const MAX_X: i32 = 101;
    const MAX_Y: i32 = 103;

    let moved_robots = robots
        .iter()
        .map(|robot| robot.move_robot(100, MAX_X, MAX_Y));

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for robot in moved_robots {
        if robot.sx < MAX_X / 2 {
            if robot.sy < MAX_Y / 2 {
                tl += 1;
            } else if robot.sy >= 1 + MAX_Y / 2 {
                bl += 1;
            }
        } else if robot.sx >= 1 + MAX_X / 2 {
            if robot.sy <= MAX_Y / 2 {
                tr += 1;
            } else if robot.sy >= 1 + MAX_Y / 2 {
                br += 1;
            }
        }
        println!("{} {} {} {} {} {}", robot.sx, robot.sy, tl, tr, bl, br);
    }

    println!("{} {} {} {} {}", tl, tr, bl, br, tl * tr * br * bl);
    println!(
        "{} {} {} {}",
        MAX_X / 2,
        1 + MAX_X / 2,
        MAX_Y / 2,
        1 + MAX_Y / 2
    )
}
fn get_data() -> Vec<Robot> {
    let re =
        Regex::new(r"p=([0-9]+),([0-9]+) v=([\-\+0-9]+),([\-\+0-9]+)").expect("Incorrect Regex");
    let file = File::open("data/day14.txt").expect("File not found");

    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|line| parse_line(&line, &re))
        .collect::<Vec<Robot>>()
}

fn parse_line(line: &str, re: &Regex) -> Robot {
    let capture = re.captures(line).unwrap();
    let values = capture
        .iter()
        .filter_map(|m| m)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect::<Vec<i32>>();
    Robot {
        sx: values[0],
        sy: values[1],
        mx: values[2],
        my: values[3],
    }
}

#[derive(Debug)]
struct Robot {
    sx: i32,
    sy: i32,
    mx: i32,
    my: i32,
}

impl Robot {
    fn move_robot(&self, sec: i32, max_x: i32, max_y: i32) -> Robot {
        let sx = (self.mx * sec + self.sx) % max_x;
        let sy = (self.my * sec + self.sy) % max_y;
        let sx = if sx < 0 { max_x + sx } else { sx };
        let sy = if sy < 0 { max_y + sy } else { sy };

        Self {
            sx,
            sy,
            mx: self.mx,
            my: self.my,
        }
    }
}
